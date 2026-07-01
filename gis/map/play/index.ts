// #region 🧲Header
/** @emoji 🗺️ GIS map play harness on `@semio-tech/framework-playground-core`. */
// #endregion 🧲Header

import {
  CommandBus,
  Controller,
  Playground,
  PLAYGROUND_NO_FIXTURE_ID,
  type PlaygroundFixtureCatalog,
  type PlaygroundFixtureHost,
  isPlaygroundNoFixtureId,
  Platform,
  AppRuntime,
  ModeRuntime,
  WindowKindRuntime,
  buildMapWindowBody,
  createPlayAppRuntime,
  createProductPlaygroundPlatform,
  createStackLayout,
  registerWindowBody,
  FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
  FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
  FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
  UI_INSPECTOR_MIXED_PLACEHOLDER,
  uiDeclarativeSectionsToTree,
  uiInspectorGroupsToTree,
  uiInspectorMixedNumber,
  uiInspectorMixedSelect,
  uiInspectorMixedText,
  uiInspectorReadonlyField,
  type UiInspectorFieldGroup,
  type CommandDescriptor,
  type WindowBodyViewContext,
  type UiNode,
  type UiSectionNode,
  type UiTreeItemNode,
  type UiTreeSectionNode,
  type WindowMeasure,
  type WindowTemplate,
  type AppTools,
  type ToolLeaf,
  toolCollection,
} from "@semio-tech/framework-playground-core";
import { Store, DocumentVcsStore, applyJsonReplaceOp, createDocumentVcsEnvelope, recordJsonProjectionChange, type JsonReplaceOp } from "@semio-tech/framework-core";

import { bootstrapElementsSurfaceChromeDocument, selectionMergeIds, type SelectionMarqueeMethod, type SelectionMergeMode } from "@semio-tech/ui-react";

import type { IconName } from "@semio-tech/ui-react";

import {
  GIS_MAP_LAYER_IDS,
  GIS_MAP_LAYER_LABEL,
  GIS_MAP_LAYER_WEIGHT_MAX,
  GIS_MAP_LAYER_WEIGHT_MIN,
  GIS_MAP_LAYER_WEIGHT_STEP,
  GIS_MAP_LOD_MODE_AUTOMATIC,
  defaultMapLayerStrokeScale,
  defaultMapLayerVisibility,
  getGisMapLodScale,
  gisMapLayerWeightSlidersAtLod,
  gisMapLodAutomaticSelectLabel,
  isGisMapLayerId,
  isGisMapLodId,
  type GisMapLayerId,
  type GisMapLodId,
  type MapDescriptor,
  type MapLayerStrokeScale,
  type MapLayerVisibility,
  type MapLodModeKind,
  type MapPositionProps,
  type MapRenderMode,
  type MapRouteProps,
  type MapVectorStyle,
  type MapHoveredFeature,
  type MapFeatureKind,
} from "@semio-tech/gis-map-react";

import reuseMapFixtureJson from "../fixture/reuse.map.gis.json";

export const GIS_MAP_PLAY_APP_ID = "gis-map-play";
export const GIS_MAP_PLAY_CONTROLLER_ID = "gis-map-play";
export const GIS_MAP_PLAY_SURFACE_ID = "gis.map.play/v1";
export const GIS_MAP_PLAY_BODY_KEY_MAIN = "gis.map.play.main";
export const GIS_MAP_PLAY_STORE_ID = "gis-map-play.snapshot";
export const GIS_MAP_PLAY_WINDOW_KIND_ID = "gis-map-main";
export const GIS_MAP_PLAY_FIXTURE_REUSE_ID = "reuse";
export const GIS_MAP_PLAY_HIERARCHY_TAB_ID = "framework.panel.hierarchy";
export const GIS_MAP_PLAY_CATALOGUE_TAB_ID = "framework.panel.catalogue";
export const GIS_MAP_PLAY_INSPECTION_TAB_ID = "framework.panel.inspection";

export type MapPlaySelectionKind = MapFeatureKind;

export type MapPlaySelectionMethod = SelectionMarqueeMethod;

export type MapPlaySelectionMode = SelectionMergeMode;

export interface GisMapFixturePositionV1 {
  readonly id: string;
  readonly lon: number;
  readonly lat: number;
  readonly label: string;
  readonly name: string;
  readonly kind: "receiver" | "donor";
  readonly icon: IconName;
  readonly sourceUrl?: string;
}

export interface GisMapFixtureRouteV1 {
  readonly id: string;
  readonly points: readonly (readonly [number, number])[];
  readonly kind: "reuse";
  readonly label?: string;
}

export interface GisMapFixtureV1 {
  readonly schema: "gis.map.fixture/v1";
  readonly name: string;
  readonly positions: readonly GisMapFixturePositionV1[];
  readonly routes: readonly GisMapFixtureRouteV1[];
  readonly regions: readonly [];
}

export const GIS_MAP_PLAY_FIXTURE_OPTIONS = [{ id: GIS_MAP_PLAY_FIXTURE_REUSE_ID, label: "Reuse map" }] as const;

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null;
}

function parseGisMapFixturePositionV1(raw: unknown): GisMapFixturePositionV1 | null {
  if (!isRecord(raw)) return null;
  if (typeof raw.id !== "string" || typeof raw.lon !== "number" || typeof raw.lat !== "number") return null;
  if (typeof raw.label !== "string" || typeof raw.name !== "string") return null;
  if (raw.kind !== "receiver" && raw.kind !== "donor") return null;
  if (typeof raw.icon !== "string") return null;
  return {
    id: raw.id,
    lon: raw.lon,
    lat: raw.lat,
    label: raw.label,
    name: raw.name,
    kind: raw.kind,
    icon: raw.icon as IconName,
    ...(typeof raw.sourceUrl === "string" ? { sourceUrl: raw.sourceUrl } : {}),
  };
}

function parseGisMapFixtureRouteV1(raw: unknown): GisMapFixtureRouteV1 | null {
  if (!isRecord(raw)) return null;
  if (typeof raw.id !== "string" || !Array.isArray(raw.points) || raw.kind !== "reuse") return null;
  const points = raw.points
    .map((row) => (Array.isArray(row) && typeof row[0] === "number" && typeof row[1] === "number" ? ([row[0], row[1]] as const) : null))
    .filter((row): row is readonly [number, number] => row !== null);
  if (points.length < 2) return null;
  return {
    id: raw.id,
    points,
    kind: "reuse",
    ...(typeof raw.label === "string" ? { label: raw.label } : {}),
  };
}

/** @emoji 🧩 Parses a GIS map fixture document. */
export function parseGisMapFixtureV1(raw: unknown): GisMapFixtureV1 | null {
  if (!isRecord(raw) || raw.schema !== "gis.map.fixture/v1" || typeof raw.name !== "string") return null;
  const positions = Array.isArray(raw.positions)
    ? raw.positions.map(parseGisMapFixturePositionV1).filter((row): row is GisMapFixturePositionV1 => row !== null)
    : [];
  const routes = Array.isArray(raw.routes)
    ? raw.routes.map(parseGisMapFixtureRouteV1).filter((row): row is GisMapFixtureRouteV1 => row !== null)
    : [];
  return { schema: "gis.map.fixture/v1", name: raw.name, positions, routes, regions: [] };
}

export const GIS_MAP_PLAY_DEFAULT_FIXTURE: GisMapFixtureV1 =
  parseGisMapFixtureV1(reuseMapFixtureJson as unknown) ?? (reuseMapFixtureJson as GisMapFixtureV1);

/** @emoji 🗺️ Maps a GIS map fixture into declarative overlay props. */
export function gisMapFixtureToDescriptor(fixture: GisMapFixtureV1): MapDescriptor {
  const positions: MapPositionProps[] = fixture.positions.map((row) => ({
    id: row.id,
    lon: row.lon,
    lat: row.lat,
    label: row.label,
    name: row.name,
    kind: row.kind,
    icon: row.icon,
    sourceUrl: row.sourceUrl,
  }));
  const routes: MapRouteProps[] = fixture.routes.map((row) => ({
    id: row.id,
    points: row.points,
  }));
  return { positions, routes, regions: [] };
}

const MAP_RENDER_MODES: readonly MapRenderMode[] = ["image", "vector", "combined"];

const GIS_MAP_RENDER_MODE_LABEL: Record<MapRenderMode, string> = {
  image: "Image",
  vector: "Vector",
  combined: "Combined",
};

const GIS_MAP_RENDER_MODE_TEMPLATES: readonly WindowTemplate[] = MAP_RENDER_MODES.map((mode) => ({
  id: `gis-map-render-${mode}`,
  label: GIS_MAP_RENDER_MODE_LABEL[mode],
  controllerId: GIS_MAP_PLAY_CONTROLLER_ID,
  command: "setRenderMode",
  args: { mode },
}));

const MAP_VECTOR_STYLES: readonly MapVectorStyle[] = ["colored", "figureGround", "invertedFigure"];

const GIS_MAP_VECTOR_STYLE_LABEL: Record<MapVectorStyle, string> = {
  colored: "Colored",
  figureGround: "Figure-Ground",
  invertedFigure: "Inverted-Figure",
};

const MAP_VECTOR_STYLE_DEFAULT_LABELS: Record<MapVectorStyle, boolean> = {
  colored: true,
  figureGround: false,
  invertedFigure: false,
};

export const GIS_MAP_PLAY_LOD_TIERS: readonly GisMapLodId[] = getGisMapLodScale().map((lod) => lod.id);

const GIS_MAP_LOD_MENU_LABEL: Record<GisMapLodId, string> = Object.fromEntries(
  getGisMapLodScale().map((lod) => [lod.id, lod.name]),
) as Record<GisMapLodId, string>;

const GIS_MAP_LAYER_ICON: Record<GisMapLayerId, string> = {
  raster: "image",
  water: "globe",
  land: "globe",
  roads: "network",
  buildings: "landmark",
  borders: "hexagon",
  labels: "tags",
  positions: "crosshair",
  positionLabels: "tags",
  routes: "arrow-right",
  regions: "layout-grid",
};

export interface MapPlaySnapshot {
  readonly renderMode: MapRenderMode;
  readonly renderModeByInstance: Readonly<Record<string, MapRenderMode>>;
  readonly vectorStyle: MapVectorStyle;
  readonly vectorStyleByInstance: Readonly<Record<string, MapVectorStyle>>;
  readonly lodMode: MapLodModeKind;
  readonly lodModeByInstance: Readonly<Record<string, MapLodModeKind>>;
  readonly layerVisibility: MapLayerVisibility;
  readonly layerVisibilityByInstance: Readonly<Record<string, MapLayerVisibility>>;
  readonly layerStrokeScale: MapLayerStrokeScale;
  readonly layerStrokeScaleByInstance: Readonly<Record<string, MapLayerStrokeScale>>;
  readonly activeFixture: GisMapFixtureV1 | null;
  readonly selectedPositionIds: readonly string[];
  readonly selectedRouteIds: readonly string[];
  readonly hoveredFeature: MapHoveredFeature | null;
  readonly selectionMode: MapPlaySelectionMode;
  readonly selectionMethod: MapPlaySelectionMethod;
  readonly fitWorldRevision: number;
}

export const GIS_MAP_PLAY_IDLE_SNAPSHOT: MapPlaySnapshot = {
  renderMode: "vector",
  renderModeByInstance: {},
  vectorStyle: "colored",
  vectorStyleByInstance: {},
  lodMode: GIS_MAP_LOD_MODE_AUTOMATIC,
  lodModeByInstance: {},
  layerVisibility: defaultMapLayerVisibility(),
  layerVisibilityByInstance: {},
  layerStrokeScale: defaultMapLayerStrokeScale(),
  layerStrokeScaleByInstance: {},
  activeFixture: null,
  selectedPositionIds: [],
  selectedRouteIds: [],
  hoveredFeature: null,
  selectionMode: "default",
  selectionMethod: "rectangle",
  fitWorldRevision: 0,
};

function mapPlayLayerWeightLabel(scale: number): string {
  return `${Math.round(scale * 100)}%`;
}

function mapPlayCmd(command: string, args?: Record<string, unknown>): CommandDescriptor {
  return { controllerId: GIS_MAP_PLAY_CONTROLLER_ID, command, args: args as never };
}

// #region 🔖MapPlayPanels
export function buildMapPlayHierarchyTree(
  fixture: GisMapFixtureV1 | null,
  selectedPositionIds: readonly string[],
  selectedRouteIds: readonly string[],
  hoveredFeature: MapHoveredFeature | null,
  hoverSink?: (payload: { featureId: string | null; featureKind: MapPlaySelectionKind | null }) => void,
): UiNode {
  if (!fixture) {
    return {
      type: "tree",
      sections: [
        {
          id: "gis-map-play-hierarchy.invalid",
          label: FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
          defaultOpen: true,
          items: [{ id: "gis-map-play-hierarchy.invalid.msg", label: "No fixture loaded" }],
        },
      ],
    };
  }
  const positionItems: UiTreeItemNode[] = fixture.positions.map((position) => ({
    id: `gis-map-play-hierarchy.position.${position.id}`,
    label: position.label || position.name || position.id,
    description: `${position.kind} · ${position.lat.toFixed(4)}, ${position.lon.toFixed(4)}`,
    command: mapPlayCmd("setSelection", { positions: [position.id], routes: [], mode: "default" }),
    onPointerEnter: hoverSink ? () => hoverSink({ featureId: position.id, featureKind: "position" }) : undefined,
    onPointerLeave: hoverSink ? () => hoverSink({ featureId: null, featureKind: null }) : undefined,
  }));
  const routeItems: UiTreeItemNode[] = fixture.routes.map((route) => ({
    id: `gis-map-play-hierarchy.route.${route.id}`,
    label: route.label || route.id,
    description: `${route.points.length} points`,
    command: mapPlayCmd("setSelection", { positions: [], routes: [route.id], mode: "default" }),
    onPointerEnter: hoverSink ? () => hoverSink({ featureId: route.id, featureKind: "route" }) : undefined,
    onPointerLeave: hoverSink ? () => hoverSink({ featureId: null, featureKind: null }) : undefined,
  }));
  const layerItems: UiTreeItemNode[] = GIS_MAP_LAYER_IDS.map((layer) => ({
    id: `gis-map-play-hierarchy.layer.${layer}`,
    label: GIS_MAP_LAYER_LABEL[layer],
    description: layer,
  }));
  const selectedIds = [
    ...selectedPositionIds.map((id) => `gis-map-play-hierarchy.position.${id}`),
    ...selectedRouteIds.map((id) => `gis-map-play-hierarchy.route.${id}`),
  ];
  const highlightedIds =
    hoveredFeature != null ? [`gis-map-play-hierarchy.${hoveredFeature.kind}.${hoveredFeature.id}`] : [];
  return {
    type: "tree",
    sections: [
      {
        id: "gis-map-play-hierarchy.layers",
        label: "Layers",
        defaultOpen: false,
        items: layerItems,
      },
      {
        id: "gis-map-play-hierarchy.positions",
        label: "Positions",
        defaultOpen: true,
        items: positionItems.length ? positionItems : [{ id: "gis-map-play-hierarchy.positions.empty", label: "(none)" }],
      },
      {
        id: "gis-map-play-hierarchy.routes",
        label: "Routes",
        defaultOpen: false,
        items: routeItems.length ? routeItems : [{ id: "gis-map-play-hierarchy.routes.empty", label: "(none)" }],
      },
    ],
    selectedIds,
    highlightedIds,
  };
}

export function buildMapPlayCatalogueTree(): UiNode {
  return {
    type: "tree",
    sections: [
      {
        id: "gis-map-play-catalogue.features",
        label: FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
        defaultOpen: true,
        items: [
          { id: "gis-map-play-catalogue.receiver", label: "Receiver position", description: "receiver" },
          { id: "gis-map-play-catalogue.donor", label: "Donor position", description: "donor" },
          { id: "gis-map-play-catalogue.route", label: "Reuse route", description: "route" },
        ],
      },
    ],
  };
}

function mapPlayInspectorPatchPositions(positionIds: readonly string[], field: string) {
  return mapPlayCmd("patchPositions", { positionIds, field });
}

function mapPlayInspectorPatchRoutes(routeIds: readonly string[], field: string) {
  return mapPlayCmd("patchRoutes", { routeIds, field });
}

function mapPlayInspectorNumberField(
  positionIds: readonly string[],
  fieldId: string,
  label: string,
  values: readonly number[],
  field: string,
): UiNode {
  const mixed = uiInspectorMixedNumber(values);
  return {
    type: "field",
    id: fieldId,
    label,
    child: {
      type: "input",
      id: `${fieldId}.input`,
      inputKind: "number",
      value: mixed.uniform ? String(mixed.value) : "",
      placeholder: mixed.uniform ? undefined : UI_INSPECTOR_MIXED_PLACEHOLDER,
      onChange: mapPlayInspectorPatchPositions(positionIds, field),
    },
  };
}

function mapPlayInspectorTextField(
  positionIds: readonly string[],
  fieldId: string,
  label: string,
  values: readonly string[],
  field: string,
): UiNode {
  const mixed = uiInspectorMixedText(values);
  return {
    type: "field",
    id: fieldId,
    label,
    child: {
      type: "input",
      id: `${fieldId}.input`,
      inputKind: "text",
      value: mixed.value,
      placeholder: mixed.placeholder,
      onChange: mapPlayInspectorPatchPositions(positionIds, field),
    },
  };
}

function mapPlayInspectorPositionGroup(positions: readonly GisMapFixturePositionV1[]): UiInspectorFieldGroup {
  const positionIds = positions.map((entry) => entry.id);
  const kindMixed = uiInspectorMixedSelect(positions.map((entry) => entry.kind));
  return {
    id: "gis-map-play-inspector.positions",
    label: positions.length === 1 ? "Position" : "Positions",
    fields: [
      mapPlayInspectorNumberField(positionIds, "gis-map-play-inspector.position.lat", "Latitude", positions.map((entry) => entry.lat), "lat"),
      mapPlayInspectorNumberField(positionIds, "gis-map-play-inspector.position.lon", "Longitude", positions.map((entry) => entry.lon), "lon"),
      mapPlayInspectorTextField(positionIds, "gis-map-play-inspector.position.label", "Label", positions.map((entry) => entry.label), "label"),
      mapPlayInspectorTextField(positionIds, "gis-map-play-inspector.position.name", "Name", positions.map((entry) => entry.name), "name"),
      {
        type: "field",
        id: "gis-map-play-inspector.position.kind",
        label: "Kind",
        child: {
          type: "select",
          id: "gis-map-play-inspector.position.kind.select",
          value: kindMixed.value,
          placeholder: kindMixed.placeholder,
          items: [
            { id: "receiver", value: "receiver", label: "Receiver" },
            { id: "donor", value: "donor", label: "Donor" },
          ],
          onChange: mapPlayInspectorPatchPositions(positionIds, "kind"),
        },
      },
    ],
  };
}

function mapPlayInspectorRouteGroup(routes: readonly GisMapFixtureRouteV1[]): UiInspectorFieldGroup {
  const routeIds = routes.map((entry) => entry.id);
  const labelMixed = uiInspectorMixedText(routes.map((entry) => entry.label ?? ""));
  return {
    id: "gis-map-play-inspector.routes",
    label: routes.length === 1 ? "Route" : "Routes",
    fields: [
      {
        type: "field",
        id: "gis-map-play-inspector.route.label",
        label: "Label",
        child: {
          type: "input",
          id: "gis-map-play-inspector.route.label.input",
          inputKind: "text",
          value: labelMixed.value,
          placeholder: labelMixed.placeholder,
          onChange: mapPlayInspectorPatchRoutes(routeIds, "label"),
        },
      },
      uiInspectorReadonlyField(
        "gis-map-play-inspector.route.points",
        "Points",
        routes.length === 1 ? String(routes[0]?.points.length ?? 0) : UI_INSPECTOR_MIXED_PLACEHOLDER,
      ),
    ],
  };
}

export function buildMapPlayInspectorTree(
  fixture: GisMapFixtureV1 | null,
  selectedPositionIds: readonly string[],
  selectedRouteIds: readonly string[],
): UiNode {
  if (!fixture) {
    return uiDeclarativeSectionsToTree([
      { type: "section", id: "gis-map-play-inspector.invalid", label: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, children: [{ type: "text", value: "No fixture loaded" }] },
    ]);
  }
  if (selectedPositionIds.length === 0 && selectedRouteIds.length === 0) {
    return uiDeclarativeSectionsToTree([
      {
        type: "section",
        id: "gis-map-play-inspector.empty",
        label: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
        children: [{ type: "text", value: "Select a position or route in the hierarchy." }],
      },
    ]);
  }
  const groups: UiInspectorFieldGroup[] = [];
  if (selectedPositionIds.length > 0) {
    const positions = selectedPositionIds
      .map((id) => fixture.positions.find((entry) => entry.id === id))
      .filter((entry): entry is GisMapFixturePositionV1 => Boolean(entry));
    if (positions.length > 0) {
      groups.push(mapPlayInspectorPositionGroup(positions));
    }
  }
  if (selectedRouteIds.length > 0) {
    const routes = selectedRouteIds
      .map((id) => fixture.routes.find((entry) => entry.id === id))
      .filter((entry): entry is GisMapFixtureRouteV1 => Boolean(entry));
    if (routes.length > 0) {
      groups.push(mapPlayInspectorRouteGroup(routes));
    }
  }
  if (!groups.length) {
    return uiDeclarativeSectionsToTree([
      { type: "section", id: "gis-map-play-inspector.missing", label: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, children: [{ type: "text", value: "Selection not found" }] },
    ]);
  }
  return uiInspectorGroupsToTree(groups);
}

export interface MapPlayToolbarState {
  readonly selectionMethod: MapPlaySelectionMethod;
  readonly selectionMode: MapPlaySelectionMode;
  readonly selectionCount: number;
}

/** @emoji 🧰 Playground toolbar tools for GIS map selection. */
export function buildMapPlayToolbarTools(state: MapPlayToolbarState, controllerId: string): AppTools {
  const selectionTools: ToolLeaf[] = [
    {
      id: "gis-map.select.rectangle",
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
      id: "gis-map.select.lasso",
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
      id: "gis-map.select.mode.default",
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
      id: "gis-map.select.mode.additive",
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
      id: "gis-map.select.mode.subtractive",
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
      id: "gis-map.select.mode.invertive",
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
      id: "gis-map.selection.clear",
      kind: "button",
      iconId: "x",
      label: "Clear",
      order: 6,
      disabled: state.selectionCount === 0,
      controllerId,
      command: "clearSelection",
    },
  ];
  return [toolCollection("selection", "mouse-pointer-2", selectionTools)];
}
// #endregion 🔖MapPlayPanels

class MapPlaySnapshotStore extends Store<MapPlaySnapshot> {
  constructor(private readonly controller: MapPlayController) {
    super();
  }

  getSnapshot(): MapPlaySnapshot {
    return this.controller.getSnapshot();
  }

  bump(): void {
    this.notify();
  }
}

export function buildMapPlayMainDeclarativeBody(_ctx: WindowBodyViewContext): UiNode {
  return buildMapWindowBody(GIS_MAP_PLAY_SURFACE_ID, GIS_MAP_PLAY_CONTROLLER_ID, "main");
}

function mapPlayRenderModeMeasure(renderMode: MapRenderMode): WindowMeasure {
  return {
    kind: "select",
    id: "gis-map-render-mode",
    label: "Tiles",
    value: renderMode,
    items: MAP_RENDER_MODES.map((mode) => ({ id: mode, value: mode, label: GIS_MAP_RENDER_MODE_LABEL[mode] })),
    onChange: mapPlayCmd("setRenderMode"),
  };
}

function mapPlayVectorStyleMeasure(vectorStyle: MapVectorStyle): WindowMeasure {
  return {
    kind: "select",
    id: "gis-map-vector-style",
    label: "Style",
    value: vectorStyle,
    items: MAP_VECTOR_STYLES.map((style) => ({
      id: style,
      value: style,
      label: GIS_MAP_VECTOR_STYLE_LABEL[style],
    })),
    onChange: mapPlayCmd("setVectorStyle"),
  };
}

function mapPlayLodMeasure(lodMode: MapLodModeKind, effectiveLodId: GisMapLodId): WindowMeasure {
  return {
    kind: "select",
    id: "gis-map-lod-mode",
    label: "LOD",
    value: lodMode,
    items: [
      {
        id: "automatic",
        value: GIS_MAP_LOD_MODE_AUTOMATIC,
        label: gisMapLodAutomaticSelectLabel(effectiveLodId),
      },
      ...GIS_MAP_PLAY_LOD_TIERS.map((tier) => ({
        id: tier,
        value: tier,
        label: GIS_MAP_LOD_MENU_LABEL[tier] ?? tier,
      })),
    ],
    onChange: mapPlayCmd("setLodMode"),
  };
}

function mapPlayLayerMeasures(
  visibility: MapLayerVisibility,
  strokeScale: MapLayerStrokeScale,
  effectiveLodId: GisMapLodId,
  renderMode: MapRenderMode,
): readonly WindowMeasure[] {
  const weightSliders = new Set(gisMapLayerWeightSlidersAtLod(effectiveLodId, renderMode));
  const out: WindowMeasure[] = [];
  for (const layer of GIS_MAP_LAYER_IDS) {
    out.push({
      kind: "toggle",
      id: `gis-map-layer-${layer}`,
      iconId: GIS_MAP_LAYER_ICON[layer],
      text: GIS_MAP_LAYER_LABEL[layer],
      pressed: visibility[layer],
      onChange: mapPlayCmd("setLayerVisible", { layer }),
    });
    if (weightSliders.has(layer)) {
      out.push({
        kind: "slider",
        id: `gis-map-layer-weight-${layer}`,
        label: mapPlayLayerWeightLabel(strokeScale[layer]),
        value: strokeScale[layer],
        min: GIS_MAP_LAYER_WEIGHT_MIN,
        max: GIS_MAP_LAYER_WEIGHT_MAX,
        step: GIS_MAP_LAYER_WEIGHT_STEP,
        onChange: mapPlayCmd("setLayerWeight", { layer }),
      });
    }
  }
  return out;
}

function mapPlayWindowMeasures(
  renderMode: MapRenderMode,
  vectorStyle: MapVectorStyle,
  lodMode: MapLodModeKind,
  effectiveLodId: GisMapLodId,
  layerVisibility: MapLayerVisibility,
  layerStrokeScale: MapLayerStrokeScale,
): readonly WindowMeasure[] {
  const displayChildren: WindowMeasure[] = [mapPlayRenderModeMeasure(renderMode)];
  if (renderMode === "vector" || renderMode === "combined") {
    displayChildren.push(mapPlayVectorStyleMeasure(vectorStyle));
  }
  displayChildren.push(mapPlayLodMeasure(lodMode, effectiveLodId));
  return [
    {
      kind: "group",
      id: "gis-map-display",
      label: "Display",
      children: displayChildren,
    },
    {
      kind: "group",
      id: "gis-map-layers",
      label: "Layers",
      defaultOpen: false,
      children: mapPlayLayerMeasures(layerVisibility, layerStrokeScale, effectiveLodId, renderMode),
    },
  ];
}

export class MapPlayController extends Controller implements PlaygroundFixtureHost {
  readonly mainMode = new ModeRuntime("explore", "Explore", undefined);
  private activeFixtureId = GIS_MAP_PLAY_FIXTURE_REUSE_ID;
  private readonly docStore = new DocumentVcsStore<GisMapFixtureV1 | null, JsonReplaceOp<GisMapFixtureV1 | null>>({
    envelope: createDocumentVcsEnvelope("gis.map.fixture/v1", "gis-map-play", GIS_MAP_PLAY_DEFAULT_FIXTURE),
    applyOp: applyJsonReplaceOp,
  });
  private readonly snapshotStore: MapPlaySnapshotStore;
  private snapshotCache: MapPlaySnapshot | null = null;
  renderMode: MapRenderMode = "vector";
  renderModeByInstance: Record<string, MapRenderMode> = {};
  vectorStyle: MapVectorStyle = "colored";
  vectorStyleByInstance: Record<string, MapVectorStyle> = {};
  lodMode: MapLodModeKind = GIS_MAP_LOD_MODE_AUTOMATIC;
  lodModeByInstance: Record<string, MapLodModeKind> = {};
  layerVisibility: MapLayerVisibility = defaultMapLayerVisibility();
  layerVisibilityByInstance: Record<string, MapLayerVisibility> = {};
  layerStrokeScale: MapLayerStrokeScale = defaultMapLayerStrokeScale();
  layerStrokeScaleByInstance: Record<string, MapLayerStrokeScale> = {};
  effectiveLodId: GisMapLodId = "world";
  effectiveLodByInstance: Record<string, GisMapLodId> = {};
  private selectedPositionIds: string[] = [];
  private selectedRouteIds: string[] = [];
  private hoveredFeature: MapHoveredFeature | null = null;
  selectionMode: MapPlaySelectionMode = "default";
  selectionMethod: MapPlaySelectionMethod = "rectangle";
  fitWorldRevision = 0;
  private interactionRevision = 0;
  private readonly snapshotListeners = new Set<() => void>();

  constructor(commandBus: CommandBus, notify: () => void) {
    super(GIS_MAP_PLAY_CONTROLLER_ID, commandBus, notify);
    this.snapshotStore = new MapPlaySnapshotStore(this);
    this.provideStore(GIS_MAP_PLAY_STORE_ID, this.snapshotStore);
    this.applyFixtureLayersForData();
    this.rebuildSnapshotCache();
    this.rebuildShellMode();
    this.rebuildToolbarTools();
  }

  /** @emoji 🗺️ Resolves tile render mode for a shell window instance (or the default window kind id). */
  getRenderModeForScope(scopeId: string): MapRenderMode {
    return this.renderModeByInstance[scopeId] ?? this.renderMode;
  }

  getVectorStyleForScope(scopeId: string): MapVectorStyle {
    return this.vectorStyleByInstance[scopeId] ?? this.vectorStyle;
  }

  getLodModeForScope(scopeId: string): MapLodModeKind {
    return this.lodModeByInstance[scopeId] ?? this.lodMode;
  }

  getEffectiveLodForScope(scopeId: string): GisMapLodId {
    return this.effectiveLodByInstance[scopeId] ?? this.effectiveLodId;
  }

  getLayerVisibilityForScope(scopeId: string): MapLayerVisibility {
    return this.layerVisibilityByInstance[scopeId] ?? this.layerVisibility;
  }

  getLayerStrokeScaleForScope(scopeId: string): MapLayerStrokeScale {
    return this.layerStrokeScaleByInstance[scopeId] ?? this.layerStrokeScale;
  }

  private rebuildSnapshotCache(): void {
    this.snapshotCache = {
      renderMode: this.renderMode,
      renderModeByInstance: { ...this.renderModeByInstance },
      vectorStyle: this.vectorStyle,
      vectorStyleByInstance: { ...this.vectorStyleByInstance },
      lodMode: this.lodMode,
      lodModeByInstance: { ...this.lodModeByInstance },
      layerVisibility: { ...this.layerVisibility },
      layerVisibilityByInstance: { ...this.layerVisibilityByInstance },
      layerStrokeScale: { ...this.layerStrokeScale },
      layerStrokeScaleByInstance: { ...this.layerStrokeScaleByInstance },
      activeFixture: this.activeFixture,
      selectedPositionIds: [...this.selectedPositionIds],
      selectedRouteIds: [...this.selectedRouteIds],
      hoveredFeature: this.hoveredFeature,
      selectionMode: this.selectionMode,
      selectionMethod: this.selectionMethod,
      fitWorldRevision: this.fitWorldRevision,
    };
  }

  getFitWorldRevision(): number {
    return this.fitWorldRevision;
  }

  getSelectedPositionIds(): readonly string[] {
    return this.selectedPositionIds;
  }

  getSelectedRouteIds(): readonly string[] {
    return this.selectedRouteIds;
  }

  getHoveredFeature(): MapHoveredFeature | null {
    return this.hoveredFeature;
  }

  getSelectionMode(): MapPlaySelectionMode {
    return this.selectionMode;
  }

  getSelectionMethod(): MapPlaySelectionMethod {
    return this.selectionMethod;
  }

  /** @emoji 🧰 Rebuilds toolbar selection tools from controller state. */
  rebuildToolbarTools(): void {
    this.mainMode.tools = buildMapPlayToolbarTools(
      {
        selectionMethod: this.selectionMethod,
        selectionMode: this.selectionMode,
        selectionCount: this.selectedPositionIds.length + this.selectedRouteIds.length,
      },
      this.id,
    );
  }

  private mergeSelection(positions: readonly string[], routes: readonly string[], mode: MapPlaySelectionMode): void {
    this.selectedPositionIds = selectionMergeIds(mode, this.selectedPositionIds, positions);
    this.selectedRouteIds = selectionMergeIds(mode, this.selectedRouteIds, routes);
  }

  private selectionCount(): number {
    return this.selectedPositionIds.length + this.selectedRouteIds.length;
  }

  private focusFeature(featureKind: MapPlaySelectionKind, featureId: string): void {
    const fixture = this.activeFixture;
    if (!fixture) return;
    if (featureKind === "position") {
      const position = fixture.positions.find((row) => row.id === featureId);
      if (!position) return;
      console.log(`[DEBUG] gis map focus position ${featureId} at ${position.lon},${position.lat}`);
      return;
    }
    const route = fixture.routes.find((row) => row.id === featureId);
    if (!route || route.points.length === 0) return;
    let lon = 0;
    let lat = 0;
    for (const [plon, plat] of route.points) {
      lon += plon;
      lat += plat;
    }
    lon /= route.points.length;
    lat /= route.points.length;
    console.log(`[DEBUG] gis map focus route ${featureId} at ${lon},${lat}`);
  }

  getSelectedFeatureId(): string | null {
    if (this.selectedPositionIds.length === 1 && this.selectedRouteIds.length === 0) {
      return this.selectedPositionIds[0] ?? null;
    }
    if (this.selectedRouteIds.length === 1 && this.selectedPositionIds.length === 0) {
      return this.selectedRouteIds[0] ?? null;
    }
    return null;
  }

  getSelectedFeatureKind(): MapPlaySelectionKind | null {
    if (this.selectedPositionIds.length === 1 && this.selectedRouteIds.length === 0) {
      return "position";
    }
    if (this.selectedRouteIds.length === 1 && this.selectedPositionIds.length === 0) {
      return "route";
    }
    return null;
  }

  getInteractionRevision(): number {
    return this.interactionRevision;
  }

  subscribeSnapshot(listener: () => void): () => void {
    this.snapshotListeners.add(listener);
    return () => this.snapshotListeners.delete(listener);
  }

  private notifySnapshot(): void {
    for (const listener of this.snapshotListeners) {
      listener();
    }
  }

  getActiveFixture(): GisMapFixtureV1 | null {
    return this.activeFixture;
  }

  getDocumentVcsStore(): DocumentVcsStore<GisMapFixtureV1 | null, JsonReplaceOp<GisMapFixtureV1 | null>> {
    return this.docStore;
  }

  private get activeFixture(): GisMapFixtureV1 | null {
    return this.docStore.projection();
  }

  private setActiveFixtureProjection(next: GisMapFixtureV1 | null): void {
    recordJsonProjectionChange(this.docStore, next);
  }

  private patchActiveFixture(nextFixture: GisMapFixtureV1): void {
    this.setActiveFixtureProjection(nextFixture);
    this.bumpSnapshot();
  }

  private patchPositionField(position: GisMapFixturePositionV1, field: string, value: unknown): GisMapFixturePositionV1 {
    if (field === "lat" || field === "lon") {
      const numeric = typeof value === "number" ? value : Number(value);
      if (!Number.isFinite(numeric)) return position;
      return { ...position, [field]: numeric };
    }
    if (field === "kind" && (value === "receiver" || value === "donor")) {
      return { ...position, kind: value };
    }
    if (typeof value !== "string") return position;
    return { ...position, [field]: value };
  }

  private patchPositions(positionIds: readonly string[], field: string, value: unknown): void {
    if (!this.activeFixture || !positionIds.length) return;
    const targets = new Set(positionIds);
    const positions = this.activeFixture.positions.map((position) =>
      targets.has(position.id) ? this.patchPositionField(position, field, value) : position,
    );
    this.patchActiveFixture({ ...this.activeFixture, positions });
  }

  private patchPosition(positionId: string, field: string, value: unknown): void {
    this.patchPositions([positionId], field, value);
  }

  private patchRoutes(routeIds: readonly string[], field: string, value: unknown): void {
    if (!this.activeFixture || !routeIds.length) return;
    const targets = new Set(routeIds);
    const routes = this.activeFixture.routes.map((route) => {
      if (!targets.has(route.id)) return route;
      if (typeof value !== "string") return route;
      return { ...route, [field]: value };
    });
    this.patchActiveFixture({ ...this.activeFixture, routes });
  }

  private patchRoute(routeId: string, field: string, value: unknown): void {
    this.patchRoutes([routeId], field, value);
  }

  getSnapshot(): MapPlaySnapshot {
    if (!this.snapshotCache) {
      this.rebuildSnapshotCache();
    }
    return this.snapshotCache!;
  }

  private bumpSnapshot(): void {
    this.rebuildSnapshotCache();
    this.snapshotStore.bump();
    this.interactionRevision += 1;
    this.notifySnapshot();
    this.rebuildToolbarTools();
    this.emit();
  }

  private rebuildShellMode(): void {
    this.mainMode.windowKinds = [
      new WindowKindRuntime(
        GIS_MAP_PLAY_WINDOW_KIND_ID,
        "World Map",
        GIS_MAP_PLAY_BODY_KEY_MAIN,
        undefined,
        mapPlayWindowMeasures(
          this.renderMode,
          this.vectorStyle,
          this.lodMode,
          this.effectiveLodId,
          this.layerVisibility,
          this.layerStrokeScale,
        ),
        undefined,
        GIS_MAP_RENDER_MODE_TEMPLATES,
      ),
    ];
  }

  getFixtureCatalog(): PlaygroundFixtureCatalog {
    return { activeFixtureId: this.activeFixtureId, options: [...GIS_MAP_PLAY_FIXTURE_OPTIONS] };
  }

  private applyFixtureLayersForData(): void {
    const visibility = { ...this.layerVisibility, positions: true, positionLabels: true, routes: true };
    this.layerVisibility = visibility;
    this.layerVisibilityByInstance = {};
  }

  run(command: string, args?: unknown): void {
    if (command === "setSelection") {
      const positions = (args as { positions?: string[] }).positions ?? [];
      const routes = (args as { routes?: string[] }).routes ?? [];
      const mode = (args as { mode?: MapPlaySelectionMode }).mode ?? "default";
      const featureId = (args as { featureId?: string | null }).featureId;
      const featureKind = (args as { featureKind?: MapPlaySelectionKind | null }).featureKind;
      if (typeof featureId === "string" && featureKind) {
        this.mergeSelection(
          featureKind === "position" ? [featureId] : [],
          featureKind === "route" ? [featureId] : [],
          mode,
        );
      } else {
        this.mergeSelection(positions, routes, mode);
      }
      this.bumpSnapshot();
      console.log(
        `[DEBUG] gis map selection positions=[${this.selectedPositionIds.join(", ")}] routes=[${this.selectedRouteIds.join(", ")}] mode=${mode}`,
      );
      return;
    }
    if (command === "setSelectionMode") {
      const mode = (args as { mode?: MapPlaySelectionMode }).mode;
      if (mode !== "default" && mode !== "additive" && mode !== "subtractive" && mode !== "invertive") return;
      if (this.selectionMode === mode) return;
      this.selectionMode = mode;
      this.bumpSnapshot();
      return;
    }
    if (command === "setSelectionMethod") {
      const method = (args as { method?: MapPlaySelectionMethod }).method;
      if (method !== "rectangle" && method !== "lasso") return;
      if (this.selectionMethod === method) return;
      this.selectionMethod = method;
      this.bumpSnapshot();
      return;
    }
    if (command === "clearSelection") {
      if (this.selectionCount() === 0) return;
      this.selectedPositionIds = [];
      this.selectedRouteIds = [];
      this.bumpSnapshot();
      return;
    }
    if (command === "setHover") {
      const featureId = (args as { featureId?: string | null }).featureId ?? null;
      const featureKind = (args as { featureKind?: MapPlaySelectionKind | null }).featureKind ?? null;
      const next =
        featureId && featureKind ? ({ kind: featureKind, id: featureId } satisfies MapHoveredFeature) : null;
      if (
        this.hoveredFeature?.id === next?.id &&
        this.hoveredFeature?.kind === next?.kind
      ) {
        return;
      }
      this.hoveredFeature = next;
      this.bumpSnapshot();
      return;
    }
    if (command === "deselect") {
      const featureId = (args as { featureId?: string }).featureId;
      const featureKind = (args as { featureKind?: MapPlaySelectionKind }).featureKind;
      if (!featureId || !featureKind) return;
      if (featureKind === "position") {
        this.selectedPositionIds = this.selectedPositionIds.filter((id) => id !== featureId);
      } else {
        this.selectedRouteIds = this.selectedRouteIds.filter((id) => id !== featureId);
      }
      this.bumpSnapshot();
      return;
    }
    if (command === "selectAll") {
      const fixture = this.activeFixture;
      if (!fixture) return;
      this.selectedPositionIds = fixture.positions.map((row) => row.id);
      this.selectedRouteIds = fixture.routes.map((row) => row.id);
      this.bumpSnapshot();
      return;
    }
    if (command === "focusFeature") {
      const featureId = (args as { featureId?: string }).featureId;
      const featureKind = (args as { featureKind?: MapPlaySelectionKind }).featureKind;
      if (!featureId || !featureKind) return;
      this.focusFeature(featureKind, featureId);
      return;
    }
    if (command === "fitWorld") {
      this.fitWorldRevision += 1;
      this.bumpSnapshot();
      console.log("[DEBUG] gis map fit world requested");
      return;
    }
    if (command === "openSource") {
      const featureId = (args as { featureId?: string }).featureId;
      const position = this.activeFixture?.positions.find((row) => row.id === featureId);
      if (position?.sourceUrl && typeof window !== "undefined") {
        window.open(position.sourceUrl, "_blank", "noopener,noreferrer");
      }
      return;
    }
    if (command === "patchPositions") {
      const positionIds = (args as { positionIds?: readonly string[] }).positionIds ?? [];
      const field = (args as { field?: string }).field;
      const value = (args as { value?: unknown }).value;
      if (positionIds.length > 0 && typeof field === "string") {
        this.patchPositions(positionIds, field, value);
      }
      return;
    }
    if (command === "patchPosition") {
      const positionId = (args as { positionId?: string }).positionId;
      const field = (args as { field?: string }).field;
      const value = (args as { value?: unknown }).value;
      if (typeof positionId === "string" && typeof field === "string") {
        this.patchPosition(positionId, field, value);
      }
      return;
    }
    if (command === "patchRoutes") {
      const routeIds = (args as { routeIds?: readonly string[] }).routeIds ?? [];
      const field = (args as { field?: string }).field;
      const value = (args as { value?: unknown }).value;
      if (routeIds.length > 0 && typeof field === "string") {
        this.patchRoutes(routeIds, field, value);
      }
      return;
    }
    if (command === "patchRoute") {
      const routeId = (args as { routeId?: string }).routeId;
      const field = (args as { field?: string }).field;
      const value = (args as { value?: unknown }).value;
      if (typeof routeId === "string" && typeof field === "string") {
        this.patchRoute(routeId, field, value);
      }
      return;
    }
    if (command === "setActiveFixture") {
      const fixtureId = (args as { fixtureId?: string }).fixtureId ?? "";
      const nextId = isPlaygroundNoFixtureId(fixtureId) ? PLAYGROUND_NO_FIXTURE_ID : fixtureId;
      if (nextId === this.activeFixtureId) return;
      this.activeFixtureId = nextId;
      if (isPlaygroundNoFixtureId(nextId)) {
        this.setActiveFixtureProjection(null);
        this.layerVisibility = defaultMapLayerVisibility();
        this.layerVisibilityByInstance = {};
        this.layerStrokeScale = defaultMapLayerStrokeScale();
        this.layerStrokeScaleByInstance = {};
        this.rebuildShellMode();
        this.bumpSnapshot();
        return;
      }
      if (nextId === GIS_MAP_PLAY_FIXTURE_REUSE_ID) {
        this.setActiveFixtureProjection(GIS_MAP_PLAY_DEFAULT_FIXTURE);
        this.applyFixtureLayersForData();
        this.rebuildShellMode();
        this.bumpSnapshot();
      }
      return;
    }
    if (command === "setRenderMode") {
      const { mode, value, instanceId } = (args ?? {}) as { mode?: string; value?: string; instanceId?: string };
      const resolved = mode ?? value;
      if (resolved !== "image" && resolved !== "vector" && resolved !== "combined") {
        return;
      }
      const scopeId = instanceId ?? GIS_MAP_PLAY_WINDOW_KIND_ID;
      this.renderModeByInstance = { ...this.renderModeByInstance, [scopeId]: resolved };
      if (scopeId === GIS_MAP_PLAY_WINDOW_KIND_ID) {
        this.renderMode = resolved;
      }
      this.rebuildShellMode();
      this.bumpSnapshot();
      return;
    }
    if (command === "setVectorStyle") {
      const { style, value, instanceId } = (args ?? {}) as { style?: string; value?: string; instanceId?: string };
      const resolved = style ?? value;
      if (resolved !== "colored" && resolved !== "figureGround" && resolved !== "invertedFigure") {
        return;
      }
      const scopeId = instanceId ?? GIS_MAP_PLAY_WINDOW_KIND_ID;
      this.vectorStyleByInstance = { ...this.vectorStyleByInstance, [scopeId]: resolved };
      if (scopeId === GIS_MAP_PLAY_WINDOW_KIND_ID) {
        this.vectorStyle = resolved;
      }
      const prevVisibility = this.getLayerVisibilityForScope(scopeId);
      const nextVisibility = { ...prevVisibility, labels: MAP_VECTOR_STYLE_DEFAULT_LABELS[resolved] };
      this.layerVisibilityByInstance = { ...this.layerVisibilityByInstance, [scopeId]: nextVisibility };
      if (scopeId === GIS_MAP_PLAY_WINDOW_KIND_ID) {
        this.layerVisibility = nextVisibility;
      }
      this.rebuildShellMode();
      this.bumpSnapshot();
      return;
    }
    if (command === "setLodMode") {
      const { mode, value, instanceId } = (args ?? {}) as { mode?: string; value?: string; instanceId?: string };
      const resolved = mode ?? value;
      if (typeof resolved !== "string") {
        return;
      }
      if (resolved !== GIS_MAP_LOD_MODE_AUTOMATIC && !isGisMapLodId(resolved)) {
        return;
      }
      const scopeId = instanceId ?? GIS_MAP_PLAY_WINDOW_KIND_ID;
      this.lodModeByInstance = { ...this.lodModeByInstance, [scopeId]: resolved };
      if (scopeId === GIS_MAP_PLAY_WINDOW_KIND_ID) {
        this.lodMode = resolved;
      }
      this.rebuildShellMode();
      this.bumpSnapshot();
      return;
    }
    if (command === "setEffectiveLod") {
      const { lod, instanceId } = (args ?? {}) as { lod?: string; instanceId?: string };
      if (typeof lod !== "string" || !isGisMapLodId(lod)) {
        return;
      }
      const scopeId = instanceId ?? GIS_MAP_PLAY_WINDOW_KIND_ID;
      const prev = this.effectiveLodByInstance[scopeId] ?? this.effectiveLodId;
      if (prev === lod && this.effectiveLodId === lod) {
        return;
      }
      this.effectiveLodByInstance = { ...this.effectiveLodByInstance, [scopeId]: lod };
      this.effectiveLodId = lod;
      this.rebuildShellMode();
      this.emit();
      return;
    }
    if (command === "setLayerVisible") {
      const { layer, pressed, value, instanceId } = (args ?? {}) as {
        layer?: string;
        pressed?: boolean;
        value?: boolean;
        instanceId?: string;
      };
      if (typeof layer !== "string" || !isGisMapLayerId(layer)) {
        return;
      }
      const visible = pressed ?? value;
      if (typeof visible !== "boolean") {
        return;
      }
      const scopeId = instanceId ?? GIS_MAP_PLAY_WINDOW_KIND_ID;
      const prev = this.getLayerVisibilityForScope(scopeId);
      if (prev[layer] === visible) {
        return;
      }
      const next = { ...prev, [layer]: visible };
      this.layerVisibilityByInstance = { ...this.layerVisibilityByInstance, [scopeId]: next };
      if (scopeId === GIS_MAP_PLAY_WINDOW_KIND_ID) {
        this.layerVisibility = next;
      }
      this.rebuildShellMode();
      this.bumpSnapshot();
      return;
    }
    if (command === "setLayerWeight") {
      const { layer, value, instanceId } = (args ?? {}) as { layer?: string; value?: number; instanceId?: string };
      if (typeof layer !== "string" || !isGisMapLayerId(layer)) {
        return;
      }
      if (typeof value !== "number" || !Number.isFinite(value)) {
        return;
      }
      const clamped = Math.min(GIS_MAP_LAYER_WEIGHT_MAX, Math.max(GIS_MAP_LAYER_WEIGHT_MIN, value));
      const scopeId = instanceId ?? GIS_MAP_PLAY_WINDOW_KIND_ID;
      const prev = this.getLayerStrokeScaleForScope(scopeId);
      if (prev[layer] === clamped) {
        return;
      }
      const next = { ...prev, [layer]: clamped };
      this.layerStrokeScaleByInstance = { ...this.layerStrokeScaleByInstance, [scopeId]: next };
      if (scopeId === GIS_MAP_PLAY_WINDOW_KIND_ID) {
        this.layerStrokeScale = next;
      }
      this.rebuildShellMode();
      this.bumpSnapshot();
    }
  }
}

function buildMapPlayAppRuntime(ctrl: MapPlayController): AppRuntime {
  const layout = createStackLayout(["gis-map-main"], ["World Map"]);
  return createPlayAppRuntime(GIS_MAP_PLAY_APP_ID, "Map", ctrl, layout, ctrl.mainMode);
}

export class PlaygroundMap extends Playground {
  readonly id = GIS_MAP_PLAY_APP_ID;

  createRuntime(): Platform {
    const runtime = createProductPlaygroundPlatform(this.id);
    const ctrl = new MapPlayController(runtime.commandBus, () => runtime.notify());
    runtime.addApp(buildMapPlayAppRuntime(ctrl));
    return runtime;
  }

  registerBodies(): void {
    registerWindowBody(GIS_MAP_PLAY_BODY_KEY_MAIN, buildMapPlayMainDeclarativeBody);
  }
}

if (
  typeof document !== "undefined" &&
  document.getElementById("root") != null &&
  !import.meta.vitest &&
  import.meta.env.PUZZLE_PLAY_ENTRY === "map"
) {
  bootstrapElementsSurfaceChromeDocument("system");
  void (async () => {
    await import("./globals.css");
    const { bootMapPlay } = await import("@semio-tech/framework-playground-renderer-react/puzzle/map");
    bootMapPlay(new PlaygroundMap());
  })();
}

if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;

  describe("buildMapPlayMainDeclarativeBody", () => {
    it("returns a gismap host surface", () => {
      const node = buildMapPlayMainDeclarativeBody({
        runtime: new Platform({ id: "test" }),
        windowKindId: "gis-map-main",
        bodyKey: GIS_MAP_PLAY_BODY_KEY_MAIN,
        activeModeId: "explore",
        generation: 0,
      });
      expect(node).toEqual({
        type: "gismap",
        componentKind: "gismap",
        surfaceId: GIS_MAP_PLAY_SURFACE_ID,
        controllerId: GIS_MAP_PLAY_CONTROLLER_ID,
        paneId: "main",
      });
    });
  });

  describe("MapPlayController render mode", () => {
    it("exposes render mode on window measures and templates", () => {
      const runtime = new Platform({ id: "test" });
      const ctrl = new MapPlayController(runtime.commandBus, () => runtime.notify());
      const kind = ctrl.mainMode.windowKinds[0];
      expect(kind?.templates?.map((row) => row.id)).toEqual(["gis-map-render-image", "gis-map-render-vector", "gis-map-render-combined"]);
      const measure = kind?.measures[0];
      expect(measure?.kind).toBe("group");
      if (measure?.kind !== "group") {
        return;
      }
      const select = measure.children[0];
      expect(select?.kind).toBe("select");
      if (select?.kind !== "select") {
        return;
      }
      expect(select.value).toBe("vector");
      expect(select.items.map((row) => row.value)).toEqual(["image", "vector", "combined"]);
      const styleSelect = measure.children[1];
      expect(styleSelect?.kind).toBe("select");
      if (styleSelect?.kind !== "select") {
        return;
      }
      expect(styleSelect.value).toBe("colored");
      const lodSelect = measure.children[2];
      expect(lodSelect?.kind).toBe("select");
      if (lodSelect?.kind !== "select") {
        return;
      }
      expect(lodSelect.value).toBe(GIS_MAP_LOD_MODE_AUTOMATIC);
      expect(lodSelect.items[0]?.value).toBe(GIS_MAP_LOD_MODE_AUTOMATIC);
      expect(lodSelect.items.map((row) => row.value)).toEqual([
        GIS_MAP_LOD_MODE_AUTOMATIC,
        ...GIS_MAP_PLAY_LOD_TIERS,
      ]);
      const layers = kind?.measures[1];
      expect(layers?.kind).toBe("group");
      if (layers?.kind !== "group") {
        return;
      }
      const weightAtWorld = gisMapLayerWeightSlidersAtLod("world", "vector");
      expect(layers.children).toHaveLength(GIS_MAP_LAYER_IDS.length + weightAtWorld.length);
      const positionsToggle = layers.children.find((row) => row.id === "gis-map-layer-positions");
      expect(positionsToggle?.kind).toBe("toggle");
      if (positionsToggle?.kind !== "toggle") {
        return;
      }
      expect(positionsToggle.pressed).toBe(true);
      expect(layers.children.find((row) => row.id === "gis-map-layer-weight-roads")).toBeUndefined();
      expect(layers.children.find((row) => row.id === "gis-map-layer-weight-raster")).toBeUndefined();
    });

    it("updates render mode from measure value and scopes by instance id", () => {
      const runtime = new Platform({ id: "test" });
      const ctrl = new MapPlayController(runtime.commandBus, () => runtime.notify());
      ctrl.run("setRenderMode", { value: "vector" });
      expect(ctrl.getSnapshot().renderMode).toBe("vector");
      expect(ctrl.getRenderModeForScope(GIS_MAP_PLAY_WINDOW_KIND_ID)).toBe("vector");
      ctrl.run("setRenderMode", { mode: "combined", instanceId: "win-gis-map-main-1" });
      expect(ctrl.getRenderModeForScope("win-gis-map-main-1")).toBe("combined");
      expect(ctrl.getRenderModeForScope(GIS_MAP_PLAY_WINDOW_KIND_ID)).toBe("vector");
    });

    it("reuses snapshot object until render mode changes", () => {
      const runtime = new Platform({ id: "test" });
      const ctrl = new MapPlayController(runtime.commandBus, () => runtime.notify());
      const before = ctrl.getSnapshot();
      expect(ctrl.getSnapshot()).toBe(before);
      ctrl.run("setRenderMode", { mode: "image" });
      expect(ctrl.getSnapshot()).not.toBe(before);
      expect(ctrl.getSnapshot().renderMode).toBe("image");
    });
  });

  describe("MapPlayController lod mode", () => {
    it("updates lod mode from measure value and scopes by instance id", () => {
      const runtime = new Platform({ id: "test" });
      const ctrl = new MapPlayController(runtime.commandBus, () => runtime.notify());
      ctrl.run("setLodMode", { value: "city" });
      expect(ctrl.getSnapshot().lodMode).toBe("city");
      expect(ctrl.getLodModeForScope(GIS_MAP_PLAY_WINDOW_KIND_ID)).toBe("city");
      ctrl.run("setLodMode", { value: "country", instanceId: "win-gis-map-main-1" });
      expect(ctrl.getLodModeForScope("win-gis-map-main-1")).toBe("country");
      expect(ctrl.getLodModeForScope(GIS_MAP_PLAY_WINDOW_KIND_ID)).toBe("city");
      ctrl.run("setLodMode", { value: GIS_MAP_LOD_MODE_AUTOMATIC });
      expect(ctrl.getLodModeForScope(GIS_MAP_PLAY_WINDOW_KIND_ID)).toBe(GIS_MAP_LOD_MODE_AUTOMATIC);
    });

    it("setEffectiveLod refreshes automatic select label", () => {
      const runtime = new Platform({ id: "test" });
      const ctrl = new MapPlayController(runtime.commandBus, () => runtime.notify());
      const before = ctrl.mainMode.windowKinds[0]?.measures[0];
      ctrl.run("setEffectiveLod", { lod: "continent" });
      const after = ctrl.mainMode.windowKinds[0]?.measures[0];
      expect(before).not.toBe(after);
      if (before?.kind !== "group" || after?.kind !== "group") {
        return;
      }
      const lodBefore = before.children[2];
      const lodAfter = after.children[2];
      if (lodBefore?.kind !== "select" || lodAfter?.kind !== "select") {
        return;
      }
      expect(lodAfter.items[0]?.label).toContain("Continent");
    });
  });

  describe("MapPlayController layer visibility", () => {
    it("updates layer visibility from toggle and scopes by instance id", () => {
      const runtime = new Platform({ id: "test" });
      const ctrl = new MapPlayController(runtime.commandBus, () => runtime.notify());
      ctrl.run("setLayerVisible", { layer: "routes", pressed: false });
      expect(ctrl.getSnapshot().layerVisibility.routes).toBe(false);
      expect(ctrl.getLayerVisibilityForScope(GIS_MAP_PLAY_WINDOW_KIND_ID).routes).toBe(false);
      ctrl.run("setLayerVisible", { layer: "regions", pressed: true, instanceId: "win-gis-map-main-1" });
      expect(ctrl.getLayerVisibilityForScope("win-gis-map-main-1").regions).toBe(true);
      ctrl.run("setLayerVisible", { layer: "regions", pressed: false, instanceId: "win-gis-map-main-1" });
      expect(ctrl.getLayerVisibilityForScope("win-gis-map-main-1").regions).toBe(false);
      expect(ctrl.getLayerVisibilityForScope(GIS_MAP_PLAY_WINDOW_KIND_ID).regions).toBe(true);
    });

    it("rebuilds layer toggle measures when visibility changes", () => {
      const runtime = new Platform({ id: "test" });
      const ctrl = new MapPlayController(runtime.commandBus, () => runtime.notify());
      const before = ctrl.mainMode.windowKinds[0]?.measures[1];
      ctrl.run("setLayerVisible", { layer: "labels", pressed: false });
      const after = ctrl.mainMode.windowKinds[0]?.measures[1];
      expect(before).not.toBe(after);
      if (before?.kind !== "group" || after?.kind !== "group") {
        return;
      }
      const labelsBefore = before.children.find((row) => row.id === "gis-map-layer-labels");
      const labelsAfter = after.children.find((row) => row.id === "gis-map-layer-labels");
      if (labelsBefore?.kind !== "toggle" || labelsAfter?.kind !== "toggle") {
        return;
      }
      expect(labelsBefore.pressed).toBe(true);
      expect(labelsAfter.pressed).toBe(false);
    });

    it("updates layer weight from slider and scopes by instance id", () => {
      const runtime = new Platform({ id: "test" });
      const ctrl = new MapPlayController(runtime.commandBus, () => runtime.notify());
      ctrl.run("setLayerWeight", { layer: "roads", value: 2 });
      expect(ctrl.getSnapshot().layerStrokeScale.roads).toBe(2);
      ctrl.run("setLayerWeight", { layer: "water", value: 0.1, instanceId: "win-gis-map-main-1" });
      expect(ctrl.getLayerStrokeScaleForScope("win-gis-map-main-1").water).toBe(GIS_MAP_LAYER_WEIGHT_MIN);
      expect(ctrl.getLayerStrokeScaleForScope(GIS_MAP_PLAY_WINDOW_KIND_ID).water).toBe(1);
    });

    it("rebuilds weight slider labels when stroke scale changes", () => {
      const runtime = new Platform({ id: "test" });
      const ctrl = new MapPlayController(runtime.commandBus, () => runtime.notify());
      const before = ctrl.mainMode.windowKinds[0]?.measures[1];
      ctrl.run("setLayerWeight", { layer: "routes", value: 1.5 });
      const after = ctrl.mainMode.windowKinds[0]?.measures[1];
      expect(before).not.toBe(after);
      if (before?.kind !== "group" || after?.kind !== "group") {
        return;
      }
      const sliderBefore = before.children.find((row) => row.id === "gis-map-layer-weight-routes");
      const sliderAfter = after.children.find((row) => row.id === "gis-map-layer-weight-routes");
      if (sliderBefore?.kind !== "slider" || sliderAfter?.kind !== "slider") {
        return;
      }
      expect(sliderBefore.label).toBe("100%");
      expect(sliderAfter.label).toBe("150%");
      expect(sliderAfter.value).toBe(1.5);
    });

    it("shows road weight slider only after effective lod reaches street", () => {
      const runtime = new Platform({ id: "test" });
      const ctrl = new MapPlayController(runtime.commandBus, () => runtime.notify());
      ctrl.run("setRenderMode", { mode: "vector" });
      const layersWorld = ctrl.mainMode.windowKinds[0]?.measures[1];
      ctrl.run("setEffectiveLod", { lod: "street" });
      const layersStreet = ctrl.mainMode.windowKinds[0]?.measures[1];
      if (layersWorld?.kind !== "group" || layersStreet?.kind !== "group") {
        return;
      }
      expect(layersWorld.children.find((row) => row.id === "gis-map-layer-weight-roads")).toBeUndefined();
      expect(layersStreet.children.find((row) => row.id === "gis-map-layer-weight-roads")?.kind).toBe("slider");
    });

    it("applies per-vector-style default labels visibility", () => {
      const runtime = new Platform({ id: "test" });
      const ctrl = new MapPlayController(runtime.commandBus, () => runtime.notify());
      expect(ctrl.getSnapshot().vectorStyle).toBe("colored");
      expect(ctrl.getSnapshot().layerVisibility.labels).toBe(true);
      ctrl.run("setVectorStyle", { style: "figureGround" });
      expect(ctrl.getSnapshot().layerVisibility.labels).toBe(false);
      ctrl.run("setVectorStyle", { style: "invertedFigure" });
      expect(ctrl.getSnapshot().layerVisibility.labels).toBe(false);
      ctrl.run("setVectorStyle", { style: "colored" });
      expect(ctrl.getSnapshot().layerVisibility.labels).toBe(true);
    });
  });

  describe("MapPlayController fixtures", () => {
    it("notifies snapshot listeners when render mode changes", () => {
      const runtime = new Platform({ id: "test" });
      const ctrl = new MapPlayController(runtime.commandBus, () => runtime.notify());
      let revision = ctrl.getInteractionRevision();
      let notifications = 0;
      const unsubscribe = ctrl.subscribeSnapshot(() => {
        notifications += 1;
        revision = ctrl.getInteractionRevision();
      });
      ctrl.run("setRenderMode", { mode: "image" });
      unsubscribe();
      expect(notifications).toBe(1);
      expect(revision).toBeGreaterThan(0);
    });

    it("loads the reuse fixture by default", () => {
      const runtime = new Platform({ id: "test" });
      const ctrl = new MapPlayController(runtime.commandBus, () => runtime.notify());
      expect(ctrl.getFixtureCatalog().activeFixtureId).toBe(GIS_MAP_PLAY_FIXTURE_REUSE_ID);
      expect(ctrl.getSnapshot().activeFixture?.schema).toBe("gis.map.fixture/v1");
      expect(ctrl.getSnapshot().activeFixture?.positions.length).toBeGreaterThan(0);
      expect(ctrl.getSnapshot().activeFixture?.routes.length).toBeGreaterThan(0);
    });

    it("clears fixture overlays when No fixture is selected", () => {
      const runtime = new Platform({ id: "test" });
      const ctrl = new MapPlayController(runtime.commandBus, () => runtime.notify());
      ctrl.run("setActiveFixture", { fixtureId: PLAYGROUND_NO_FIXTURE_ID });
      expect(ctrl.getSnapshot().activeFixture).toBeNull();
    });

    it("maps fixture positions into a map descriptor", () => {
      const descriptor = gisMapFixtureToDescriptor(GIS_MAP_PLAY_DEFAULT_FIXTURE);
      expect(descriptor.positions[0]?.sourceUrl).toBeTruthy();
      expect(descriptor.routes.length).toBe(GIS_MAP_PLAY_DEFAULT_FIXTURE.routes.length);
    });

    it("merges multi-kind selection with additive mode", () => {
      const runtime = new Platform({ id: "test" });
      const ctrl = new MapPlayController(runtime.commandBus, () => runtime.notify());
      ctrl.run("setSelection", { positions: ["a"], routes: [], mode: "default" });
      ctrl.run("setSelection", { positions: ["b"], routes: ["r1"], mode: "additive" });
      expect(ctrl.getSelectedPositionIds()).toEqual(["a", "b"]);
      expect(ctrl.getSelectedRouteIds()).toEqual(["r1"]);
    });

    it("buildMapPlayToolbarTools registers selection controls", () => {
      const tools = buildMapPlayToolbarTools(
        { selectionMethod: "rectangle", selectionMode: "default", selectionCount: 0 },
        GIS_MAP_PLAY_CONTROLLER_ID,
      );
      expect(tools.selection?.length).toBeGreaterThan(0);
    });

    it("buildMapPlayInspectorTree batches position fields for multi-select", () => {
      const fixture = GIS_MAP_PLAY_DEFAULT_FIXTURE;
      const positionIds = fixture.positions.slice(0, 2).map((row) => row.id);
      const tree = buildMapPlayInspectorTree(fixture, positionIds, []);
      expect(tree.type).toBe("tree");
      const latField = tree.sections[0]?.items.find((item) => item.id === "gis-map-play-inspector.position.lat");
      expect(latField?.control?.type).toBe("input");
      expect(latField?.control?.onChange?.command).toBe("patchPositions");
      expect(latField?.control?.onChange?.args).toMatchObject({ positionIds, field: "lat" });
    });

    it("patchPositions updates every selected position", () => {
      const runtime = new Platform({ id: "test" });
      const ctrl = new MapPlayController(runtime.commandBus, () => runtime.notify());
      const fixture = ctrl.getActiveFixture();
      expect(fixture).not.toBeNull();
      const positionIds = fixture!.positions.slice(0, 2).map((row) => row.id);
      ctrl.run("patchPositions", { positionIds, field: "label", value: "batch-label" });
      const updated = ctrl.getActiveFixture()!;
      for (const positionId of positionIds) {
        expect(updated.positions.find((row) => row.id === positionId)?.label).toBe("batch-label");
      }
    });
  });
}
