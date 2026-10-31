/** 🧬️ BIM model snapshot schema — authored parameters only; nothing derivable is stored. */
import { parseAnnotationStyle, parseAreaScheme, parseBeam, parseBeamType, parseBuilding, parseCeiling, parseCeilingType, parseClashSet, parseClassificationSet, parseClassificationSystem, parseColumn, parseColumnType, parseComponent, parseComponentOverride, parseCurtainPanelOverride, parseCurtainWall, parseCurtainWallType, parseDimension, parseDoorType, parseFamily, parseFamilyParameter, parseFamilySolid, parseGridLine, parseIssue, parseIssueComment, parseLeader, parseLineLocation as parseLineLocation$a4dfa9739780c6d073fc74bcb99988246e24e6c3e1971cf9010a5e6f6c74099f, parseLoadCase as parseLoadCase$a4dfa9739780c6d073fc74bcb99988246e24e6c3e1971cf9010a5e6f6c74099f, parseMaterial, parseMepElement, parseOpening, parsePoint3, parsePointLocation as parsePointLocation$a4dfa9739780c6d073fc74bcb99988246e24e6c3e1971cf9010a5e6f6c74099f, parseProject, parsePropertySet, parsePropertyTemplate, parseRailing, parseRamp, parseRestraints as parseRestraints$a4dfa9739780c6d073fc74bcb99988246e24e6c3e1971cf9010a5e6f6c74099f, parseRoof, parseRoofType, parseRule, parseSchedule, parseSheet, parseSheetRevision, parseSite, parseSlab, parseSlabType, parseSpace, parseSpaceConditions, parseStair, parseStorey, parseStructuralLoad as parseStructuralLoad$a4dfa9739780c6d073fc74bcb99988246e24e6c3e1971cf9010a5e6f6c74099f, parseStructuralLocation as parseStructuralLocation$a4dfa9739780c6d073fc74bcb99988246e24e6c3e1971cf9010a5e6f6c74099f, parseStructuralSupport as parseStructuralSupport$a4dfa9739780c6d073fc74bcb99988246e24e6c3e1971cf9010a5e6f6c74099f, parseTag, parseTextNote, parseView, parseViewport, parseWall, parseWallSweep, parseWallType, parseWindowType, parseZone } from "../🟦️.ts";

import type { OptionGroup, DesignOption, Workset, ElementMembership, PropertyKind, TemplateTarget, AnchorEnd, WallSide, TagCategory, Terminator, DimensionUnit, FamilyCategory, ParameterKind, SolidAxis, MepSystem, ElementClass, RuleKind, RuleSeverity, IssueStatus, IssuePriority, IsoSize, Orientation, MaterialCategory, LayerFunction, LocationLine, Phase, DoorLeaves, Swing, Turn, StringerKind, RiserKind, ScheduleCategory, ScheduleField, ScheduleOp, EndJoin, ViewKind, DetailLevel, ViewCategory, HostSide, AreaMeasure, AnnotationAnchor, ParametricProfile, SolidShape, MepShape, Paper, CurtainGrid, CurtainPanel, Axis, TopConstraint, Profile, RoofShape, OpeningKind, StairFlight, PropertyValue, Infill, SpaceBoundary, ScheduleKey, PropertyDef, PropertyTemplate, ClassificationItem, ClassificationSystem, SpaceConditions, Dimension, Tag, TextNote, Leader, AnnotationStyle, ExprPoint, ExprPoint3, Family, FamilyParameter, FamilySolid, Point3, Component, ComponentOverride, MepElement, ElementSelector, RuleScope, ClashRef, SectionBox, IssueViewpoint, ClashSet, Rule, Issue, IssueComment, Sheet, Viewport, SheetRevision, WallSweep, Point2, Rgb, Vertex, Slope, StairStringer, Baluster, RailingHost, Layer, Project, Material, WallType, SlabType, CeilingType, RoofType, ColumnType, BeamType, WindowType, DoorType, Site, Building, Storey, GridLine, Wall, CurtainWallType, CurtainPanelOverride, CurtainWall, Column, Beam, Slab, Ceiling, Roof, Opening, Stair, Railing, Ramp, Space, Zone, AreaScheme, ViewPlane, ViewCrop, ViewCamera, View, ScheduleColumn, ScheduleSort, ScheduleFilter, ScheduleGroup, Schedule, PropertySet, ClassificationSet } from "../🟦️.ts";

export interface ModelSnapshot {
  /** 📐️ @state artifact */
  schema: string;
  /** 🏗️ @state artifact */
  project: Project;
  /** 🧱️ @state artifact */
  materials?: Record<string, Material>;
  /** 🚧️ @state artifact */
  wall_types?: Record<string, WallType>;
  /** 🟫️ @state artifact */
  slab_types?: Record<string, SlabType>;
  /** 🏠️ @state artifact */
  roof_types?: Record<string, RoofType>;
  /** 🏛️ @state artifact */
  column_types?: Record<string, ColumnType>;
  /** 🪵️ @state artifact */
  beam_types?: Record<string, BeamType>;
  /** 🪟️ @state artifact */
  window_types?: Record<string, WindowType>;
  /** 🚪️ @state artifact */
  door_types?: Record<string, DoorType>;
  /** 🎭️ @state artifact */
  curtain_wall_types?: Record<string, CurtainWallType>;
  /** 🏢️ @state artifact */
  sites?: Record<string, Site>;
  /** 🏘️ @state artifact */
  buildings?: Record<string, Building>;
  /** 🪜️ @state artifact */
  storeys?: Record<string, Storey>;
  /** 🔳️ @state artifact */
  grids?: Record<string, GridLine>;
  /** 🪨️ @state artifact */
  walls?: Record<string, Wall>;
  /** 🧵️ @state artifact */
  curtain_walls?: Record<string, CurtainWall>;
  /** 🕳️ @state artifact */
  curtain_panel_overrides?: Record<string, CurtainPanelOverride>;
  /** 🗂️ @state artifact */
  columns?: Record<string, Column>;
  /** 📏️ @state artifact */
  beams?: Record<string, Beam>;
  /** ⬜️ @state artifact */
  slabs?: Record<string, Slab>;
  /** 🛖️ @state artifact */
  roofs?: Record<string, Roof>;
  /** 🔲️ @state artifact */
  openings?: Record<string, Opening>;
  /** 🛗️ @state artifact */
  stairs?: Record<string, Stair>;
  /** 🛤️ @state artifact */
  railings?: Record<string, Railing>;
  /** ♿️ @state artifact */
  ramps?: Record<string, Ramp>;
  /** 🛋️ @state artifact */
  spaces?: Record<string, Space>;
  /** 🔝️ @state artifact */
  ceiling_types?: Record<string, CeilingType>;
  /** 💡️ @state artifact */
  ceilings?: Record<string, Ceiling>;
  /** 🛏️ @state artifact */
  zones?: Record<string, Zone>;
  /** 🗺️ @state artifact */
  area_schemes?: Record<string, AreaScheme>;
  /** 🌌️ @state artifact */
  space_conditions?: Record<string, SpaceConditions>;
  /** 📷️ @state artifact */
  views?: Record<string, View>;
  /** 📄️ @state artifact */
  sheets?: Record<string, Sheet>;
  /** 🖼️ @state artifact */
  viewports?: Record<string, Viewport>;
  /** 📃️ @state artifact */
  sheet_revisions?: Record<string, SheetRevision>;
  /** ↔️ @state artifact */
  dimensions?: Record<string, Dimension>;
  /** 🏷️ @state artifact */
  tags?: Record<string, Tag>;
  /** 💬️ @state artifact */
  text_notes?: Record<string, TextNote>;
  /** ➡️ @state artifact */
  leaders?: Record<string, Leader>;
  /** ✍️ @state artifact */
  annotation_styles?: Record<string, AnnotationStyle>;
  /** 👪️ @state artifact */
  families?: Record<string, Family>;
  /** 🌳️ @state artifact */
  family_parameters?: Record<string, FamilyParameter>;
  /** 🪢️ @state artifact */
  family_solids?: Record<string, FamilySolid>;
  /** 🔌️ @state artifact */
  components?: Record<string, Component>;
  /** 🧩️ @state artifact */
  component_overrides?: Record<string, ComponentOverride>;
  /** ⚡️ @state artifact */
  mep_elements?: Record<string, MepElement>;
  /** 💥️ @state artifact */
  clash_sets?: Record<string, ClashSet>;
  /** 📜️ @state artifact */
  rules?: Record<string, Rule>;
  /** 🩺️ @state artifact */
  issues?: Record<string, Issue>;
  /** 🚨️ @state artifact */
  issue_comments?: Record<string, IssueComment>;
  /** 🧹️ @state artifact */
  wall_sweeps?: Record<string, WallSweep>;
  /** ⏱️ @state artifact */
  schedules?: Record<string, Schedule>;
  /** 📇️ @state artifact */
  property_templates?: Record<string, PropertyTemplate>;
  /** 📚️ @state artifact */
  classification_systems?: Record<string, ClassificationSystem>;
  /** 🎛️ @state artifact */
  properties?: Record<string, PropertySet>;
  /** 🚦️ @state artifact */
  classifications?: Record<string, ClassificationSet>;
  option_groups?: Record<string, OptionGroup>;
  design_options?: Record<string, DesignOption>;
  worksets?: Record<string, Workset>;
  element_options?: Record<string, ElementMembership>;
  element_worksets?: Record<string, ElementMembership>;
}


import type {StructuralSupport,LoadCase,StructuralLoad} from "../🟦️.ts";
export interface ModelSnapshot { supports?: Record<string,StructuralSupport>; load_cases?: Record<string,LoadCase>; loads?: Record<string,StructuralLoad>; }

//#region 🚪️Parsers
/** 🛑️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class bimModelSnapshotGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const bimModelSnapshotGuardReject = (at: string, why: string): never => {
  throw new bimModelSnapshotGuardRefusal(at, why);
};

type bimModelSnapshotGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type bimModelSnapshotGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number; readonly exclusiveMinimum?: number; readonly exclusiveMaximum?: number };
type bimModelSnapshotGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const bimModelSnapshotGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : bimModelSnapshotGuardReject(at, "value is not an object");
export const bimModelSnapshotGuardClosed = (row: Readonly<Record<string, unknown>>, at: string, keys: readonly string[]): Readonly<Record<string, unknown>> => {
  for (const key of Object.keys(row)) if (!keys.includes(key)) bimModelSnapshotGuardReject(`${at}.${key}`, "unknown property");
  return row;
};
export const bimModelSnapshotGuardExactObject = (value: unknown, at: string, fields: readonly string[], required: readonly string[] = fields): Readonly<Record<string, unknown>> => {
  const row = bimModelSnapshotGuardClosed(bimModelSnapshotGuardObject(value, at), at, fields);
  for (const field of required) if (!Object.hasOwn(row, field)) bimModelSnapshotGuardReject(`${at}.${field}`, "field is missing");
  return row;
};
export const bimModelSnapshotGuardArray = (value: unknown, at: string, bounds: bimModelSnapshotGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return bimModelSnapshotGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) bimModelSnapshotGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) bimModelSnapshotGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const bimModelSnapshotGuardString = (value: unknown, at: string, bounds: bimModelSnapshotGuardTextBounds = {}): string => {
  if (typeof value !== "string") return bimModelSnapshotGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) bimModelSnapshotGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) bimModelSnapshotGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) bimModelSnapshotGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const bimModelSnapshotGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : bimModelSnapshotGuardReject(at, "value is not a boolean"));
export const bimModelSnapshotGuardNumber = (value: unknown, at: string, bounds: bimModelSnapshotGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return bimModelSnapshotGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) bimModelSnapshotGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) bimModelSnapshotGuardReject(at, `number is above ${bounds.maximum}`);
  if (bounds.exclusiveMinimum !== undefined && value <= bounds.exclusiveMinimum) bimModelSnapshotGuardReject(at, `number is not above ${bounds.exclusiveMinimum}`);
  if (bounds.exclusiveMaximum !== undefined && value >= bounds.exclusiveMaximum) bimModelSnapshotGuardReject(at, `number is not below ${bounds.exclusiveMaximum}`);
  return value;
};
export const bimModelSnapshotGuardInteger = (value: unknown, at: string, bounds: bimModelSnapshotGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? bimModelSnapshotGuardNumber(value, at, bounds) : bimModelSnapshotGuardReject(at, "value is not an integer");
export const bimModelSnapshotGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : bimModelSnapshotGuardReject(at, `value is not one of ${members.join(", ")}`);
export const bimModelSnapshotGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : bimModelSnapshotGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export type { DesignOption } from "../🟦️.ts";

export function parseDesignOption(value: unknown, at = "$"): DesignOption {
  const row = bimModelSnapshotGuardClosed(bimModelSnapshotGuardObject(value, at), at, ["group", "name", "primary"]);
  return {
    group: bimModelSnapshotGuardString(row["group"], `${at}.group`),
    name: bimModelSnapshotGuardString(row["name"], `${at}.name`, {"minLength": 1}),
    primary: bimModelSnapshotGuardBoolean(row["primary"], `${at}.primary`),
  };
}

export type { ElementMembership } from "../🟦️.ts";

export function parseElementMembership(value: unknown, at = "$"): ElementMembership {
  const row = bimModelSnapshotGuardClosed(bimModelSnapshotGuardObject(value, at), at, ["target"]);
  return {
    target: bimModelSnapshotGuardString(row["target"], `${at}.target`),
  };
}

export interface LineLocation {
  readonly start: number;
  readonly end: number;
}

export function parseLineLocation(value: unknown, at = "$"): LineLocation {
  const row = bimModelSnapshotGuardClosed(bimModelSnapshotGuardObject(value, at), at, ["start", "end"]);
  return {
    start: bimModelSnapshotGuardNumber(row["start"], `${at}.start`, {"minimum": 0, "maximum": 1}),
    end: bimModelSnapshotGuardNumber(row["end"], `${at}.end`, {"minimum": 0, "maximum": 1}),
  };
}

export type { LoadCase } from "../🟦️.ts";

export function parseLoadCase(value: unknown, at = "$"): LoadCase {
  const row = bimModelSnapshotGuardClosed(bimModelSnapshotGuardObject(value, at), at, ["name", "category", "factor"]);
  return {
    name: bimModelSnapshotGuardString(row["name"], `${at}.name`),
    category: bimModelSnapshotGuardString(row["category"], `${at}.category`),
    factor: bimModelSnapshotGuardNumber(row["factor"], `${at}.factor`),
  };
}

export function parseModelSnapshot(value: unknown, at = "$"): ModelSnapshot {
  const row = bimModelSnapshotGuardClosed(bimModelSnapshotGuardObject(value, at), at, ["schema", "project", "materials", "wall_types", "slab_types", "roof_types", "column_types", "beam_types", "window_types", "door_types", "curtain_wall_types", "sites", "buildings", "storeys", "grids", "walls", "curtain_walls", "curtain_panel_overrides", "columns", "beams", "slabs", "roofs", "openings", "stairs", "railings", "ramps", "spaces", "ceiling_types", "ceilings", "zones", "area_schemes", "space_conditions", "views", "sheets", "viewports", "sheet_revisions", "dimensions", "tags", "text_notes", "leaders", "annotation_styles", "families", "family_parameters", "family_solids", "components", "component_overrides", "mep_elements", "clash_sets", "rules", "issues", "issue_comments", "wall_sweeps", "schedules", "property_templates", "classification_systems", "properties", "classifications", "option_groups", "design_options", "worksets", "element_options", "element_worksets", "supports", "load_cases", "loads"]);
  return {
    schema: bimModelSnapshotGuardString(row["schema"], `${at}.schema`),
    project: parseProject(row["project"], `${at}.project`),
    materials: row["materials"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelSnapshotGuardObject(row["materials"], `${at}.materials`)).map(([key0, value0]) => [key0, parseMaterial(value0, `${at}.materials.${key0}`)])),
    wall_types: row["wall_types"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelSnapshotGuardObject(row["wall_types"], `${at}.wall_types`)).map(([key0, value0]) => [key0, parseWallType(value0, `${at}.wall_types.${key0}`)])),
    slab_types: row["slab_types"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelSnapshotGuardObject(row["slab_types"], `${at}.slab_types`)).map(([key0, value0]) => [key0, parseSlabType(value0, `${at}.slab_types.${key0}`)])),
    roof_types: row["roof_types"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelSnapshotGuardObject(row["roof_types"], `${at}.roof_types`)).map(([key0, value0]) => [key0, parseRoofType(value0, `${at}.roof_types.${key0}`)])),
    column_types: row["column_types"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelSnapshotGuardObject(row["column_types"], `${at}.column_types`)).map(([key0, value0]) => [key0, parseColumnType(value0, `${at}.column_types.${key0}`)])),
    beam_types: row["beam_types"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelSnapshotGuardObject(row["beam_types"], `${at}.beam_types`)).map(([key0, value0]) => [key0, parseBeamType(value0, `${at}.beam_types.${key0}`)])),
    window_types: row["window_types"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelSnapshotGuardObject(row["window_types"], `${at}.window_types`)).map(([key0, value0]) => [key0, parseWindowType(value0, `${at}.window_types.${key0}`)])),
    door_types: row["door_types"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelSnapshotGuardObject(row["door_types"], `${at}.door_types`)).map(([key0, value0]) => [key0, parseDoorType(value0, `${at}.door_types.${key0}`)])),
    curtain_wall_types: row["curtain_wall_types"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelSnapshotGuardObject(row["curtain_wall_types"], `${at}.curtain_wall_types`)).map(([key0, value0]) => [key0, parseCurtainWallType(value0, `${at}.curtain_wall_types.${key0}`)])),
    sites: row["sites"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelSnapshotGuardObject(row["sites"], `${at}.sites`)).map(([key0, value0]) => [key0, parseSite(value0, `${at}.sites.${key0}`)])),
    buildings: row["buildings"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelSnapshotGuardObject(row["buildings"], `${at}.buildings`)).map(([key0, value0]) => [key0, parseBuilding(value0, `${at}.buildings.${key0}`)])),
    storeys: row["storeys"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelSnapshotGuardObject(row["storeys"], `${at}.storeys`)).map(([key0, value0]) => [key0, parseStorey(value0, `${at}.storeys.${key0}`)])),
    grids: row["grids"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelSnapshotGuardObject(row["grids"], `${at}.grids`)).map(([key0, value0]) => [key0, parseGridLine(value0, `${at}.grids.${key0}`)])),
    walls: row["walls"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelSnapshotGuardObject(row["walls"], `${at}.walls`)).map(([key0, value0]) => [key0, parseWall(value0, `${at}.walls.${key0}`)])),
    curtain_walls: row["curtain_walls"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelSnapshotGuardObject(row["curtain_walls"], `${at}.curtain_walls`)).map(([key0, value0]) => [key0, parseCurtainWall(value0, `${at}.curtain_walls.${key0}`)])),
    curtain_panel_overrides: row["curtain_panel_overrides"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelSnapshotGuardObject(row["curtain_panel_overrides"], `${at}.curtain_panel_overrides`)).map(([key0, value0]) => [key0, parseCurtainPanelOverride(value0, `${at}.curtain_panel_overrides.${key0}`)])),
    columns: row["columns"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelSnapshotGuardObject(row["columns"], `${at}.columns`)).map(([key0, value0]) => [key0, parseColumn(value0, `${at}.columns.${key0}`)])),
    beams: row["beams"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelSnapshotGuardObject(row["beams"], `${at}.beams`)).map(([key0, value0]) => [key0, parseBeam(value0, `${at}.beams.${key0}`)])),
    slabs: row["slabs"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelSnapshotGuardObject(row["slabs"], `${at}.slabs`)).map(([key0, value0]) => [key0, parseSlab(value0, `${at}.slabs.${key0}`)])),
    roofs: row["roofs"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelSnapshotGuardObject(row["roofs"], `${at}.roofs`)).map(([key0, value0]) => [key0, parseRoof(value0, `${at}.roofs.${key0}`)])),
    openings: row["openings"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelSnapshotGuardObject(row["openings"], `${at}.openings`)).map(([key0, value0]) => [key0, parseOpening(value0, `${at}.openings.${key0}`)])),
    stairs: row["stairs"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelSnapshotGuardObject(row["stairs"], `${at}.stairs`)).map(([key0, value0]) => [key0, parseStair(value0, `${at}.stairs.${key0}`)])),
    railings: row["railings"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelSnapshotGuardObject(row["railings"], `${at}.railings`)).map(([key0, value0]) => [key0, parseRailing(value0, `${at}.railings.${key0}`)])),
    ramps: row["ramps"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelSnapshotGuardObject(row["ramps"], `${at}.ramps`)).map(([key0, value0]) => [key0, parseRamp(value0, `${at}.ramps.${key0}`)])),
    spaces: row["spaces"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelSnapshotGuardObject(row["spaces"], `${at}.spaces`)).map(([key0, value0]) => [key0, parseSpace(value0, `${at}.spaces.${key0}`)])),
    ceiling_types: row["ceiling_types"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelSnapshotGuardObject(row["ceiling_types"], `${at}.ceiling_types`)).map(([key0, value0]) => [key0, parseCeilingType(value0, `${at}.ceiling_types.${key0}`)])),
    ceilings: row["ceilings"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelSnapshotGuardObject(row["ceilings"], `${at}.ceilings`)).map(([key0, value0]) => [key0, parseCeiling(value0, `${at}.ceilings.${key0}`)])),
    zones: row["zones"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelSnapshotGuardObject(row["zones"], `${at}.zones`)).map(([key0, value0]) => [key0, parseZone(value0, `${at}.zones.${key0}`)])),
    area_schemes: row["area_schemes"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelSnapshotGuardObject(row["area_schemes"], `${at}.area_schemes`)).map(([key0, value0]) => [key0, parseAreaScheme(value0, `${at}.area_schemes.${key0}`)])),
    space_conditions: row["space_conditions"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelSnapshotGuardObject(row["space_conditions"], `${at}.space_conditions`)).map(([key0, value0]) => [key0, parseSpaceConditions(value0, `${at}.space_conditions.${key0}`)])),
    views: row["views"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelSnapshotGuardObject(row["views"], `${at}.views`)).map(([key0, value0]) => [key0, parseView(value0, `${at}.views.${key0}`)])),
    sheets: row["sheets"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelSnapshotGuardObject(row["sheets"], `${at}.sheets`)).map(([key0, value0]) => [key0, parseSheet(value0, `${at}.sheets.${key0}`)])),
    viewports: row["viewports"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelSnapshotGuardObject(row["viewports"], `${at}.viewports`)).map(([key0, value0]) => [key0, parseViewport(value0, `${at}.viewports.${key0}`)])),
    sheet_revisions: row["sheet_revisions"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelSnapshotGuardObject(row["sheet_revisions"], `${at}.sheet_revisions`)).map(([key0, value0]) => [key0, parseSheetRevision(value0, `${at}.sheet_revisions.${key0}`)])),
    dimensions: row["dimensions"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelSnapshotGuardObject(row["dimensions"], `${at}.dimensions`)).map(([key0, value0]) => [key0, parseDimension(value0, `${at}.dimensions.${key0}`)])),
    tags: row["tags"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelSnapshotGuardObject(row["tags"], `${at}.tags`)).map(([key0, value0]) => [key0, parseTag(value0, `${at}.tags.${key0}`)])),
    text_notes: row["text_notes"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelSnapshotGuardObject(row["text_notes"], `${at}.text_notes`)).map(([key0, value0]) => [key0, parseTextNote(value0, `${at}.text_notes.${key0}`)])),
    leaders: row["leaders"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelSnapshotGuardObject(row["leaders"], `${at}.leaders`)).map(([key0, value0]) => [key0, parseLeader(value0, `${at}.leaders.${key0}`)])),
    annotation_styles: row["annotation_styles"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelSnapshotGuardObject(row["annotation_styles"], `${at}.annotation_styles`)).map(([key0, value0]) => [key0, parseAnnotationStyle(value0, `${at}.annotation_styles.${key0}`)])),
    families: row["families"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelSnapshotGuardObject(row["families"], `${at}.families`)).map(([key0, value0]) => [key0, parseFamily(value0, `${at}.families.${key0}`)])),
    family_parameters: row["family_parameters"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelSnapshotGuardObject(row["family_parameters"], `${at}.family_parameters`)).map(([key0, value0]) => [key0, parseFamilyParameter(value0, `${at}.family_parameters.${key0}`)])),
    family_solids: row["family_solids"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelSnapshotGuardObject(row["family_solids"], `${at}.family_solids`)).map(([key0, value0]) => [key0, parseFamilySolid(value0, `${at}.family_solids.${key0}`)])),
    components: row["components"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelSnapshotGuardObject(row["components"], `${at}.components`)).map(([key0, value0]) => [key0, parseComponent(value0, `${at}.components.${key0}`)])),
    component_overrides: row["component_overrides"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelSnapshotGuardObject(row["component_overrides"], `${at}.component_overrides`)).map(([key0, value0]) => [key0, parseComponentOverride(value0, `${at}.component_overrides.${key0}`)])),
    mep_elements: row["mep_elements"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelSnapshotGuardObject(row["mep_elements"], `${at}.mep_elements`)).map(([key0, value0]) => [key0, parseMepElement(value0, `${at}.mep_elements.${key0}`)])),
    clash_sets: row["clash_sets"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelSnapshotGuardObject(row["clash_sets"], `${at}.clash_sets`)).map(([key0, value0]) => [key0, parseClashSet(value0, `${at}.clash_sets.${key0}`)])),
    rules: row["rules"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelSnapshotGuardObject(row["rules"], `${at}.rules`)).map(([key0, value0]) => [key0, parseRule(value0, `${at}.rules.${key0}`)])),
    issues: row["issues"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelSnapshotGuardObject(row["issues"], `${at}.issues`)).map(([key0, value0]) => [key0, parseIssue(value0, `${at}.issues.${key0}`)])),
    issue_comments: row["issue_comments"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelSnapshotGuardObject(row["issue_comments"], `${at}.issue_comments`)).map(([key0, value0]) => [key0, parseIssueComment(value0, `${at}.issue_comments.${key0}`)])),
    wall_sweeps: row["wall_sweeps"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelSnapshotGuardObject(row["wall_sweeps"], `${at}.wall_sweeps`)).map(([key0, value0]) => [key0, parseWallSweep(value0, `${at}.wall_sweeps.${key0}`)])),
    schedules: row["schedules"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelSnapshotGuardObject(row["schedules"], `${at}.schedules`)).map(([key0, value0]) => [key0, parseSchedule(value0, `${at}.schedules.${key0}`)])),
    property_templates: row["property_templates"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelSnapshotGuardObject(row["property_templates"], `${at}.property_templates`)).map(([key0, value0]) => [key0, parsePropertyTemplate(value0, `${at}.property_templates.${key0}`)])),
    classification_systems: row["classification_systems"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelSnapshotGuardObject(row["classification_systems"], `${at}.classification_systems`)).map(([key0, value0]) => [key0, parseClassificationSystem(value0, `${at}.classification_systems.${key0}`)])),
    properties: row["properties"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelSnapshotGuardObject(row["properties"], `${at}.properties`)).map(([key0, value0]) => [key0, parsePropertySet(value0, `${at}.properties.${key0}`)])),
    classifications: row["classifications"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelSnapshotGuardObject(row["classifications"], `${at}.classifications`)).map(([key0, value0]) => [key0, parseClassificationSet(value0, `${at}.classifications.${key0}`)])),
    option_groups: row["option_groups"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelSnapshotGuardObject(row["option_groups"], `${at}.option_groups`)).map(([key0, value0]) => [key0, parseOptionGroup(value0, `${at}.option_groups.${key0}`)])),
    design_options: row["design_options"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelSnapshotGuardObject(row["design_options"], `${at}.design_options`)).map(([key0, value0]) => [key0, parseDesignOption(value0, `${at}.design_options.${key0}`)])),
    worksets: row["worksets"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelSnapshotGuardObject(row["worksets"], `${at}.worksets`)).map(([key0, value0]) => [key0, parseWorkset(value0, `${at}.worksets.${key0}`)])),
    element_options: row["element_options"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelSnapshotGuardObject(row["element_options"], `${at}.element_options`)).map(([key0, value0]) => [key0, parseElementMembership(value0, `${at}.element_options.${key0}`)])),
    element_worksets: row["element_worksets"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelSnapshotGuardObject(row["element_worksets"], `${at}.element_worksets`)).map(([key0, value0]) => [key0, parseElementMembership(value0, `${at}.element_worksets.${key0}`)])),
    supports: row["supports"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelSnapshotGuardObject(row["supports"], `${at}.supports`)).map(([key0, value0]) => [key0, parseStructuralSupport$a4dfa9739780c6d073fc74bcb99988246e24e6c3e1971cf9010a5e6f6c74099f(value0, `${at}.supports.${key0}`)])),
    load_cases: row["load_cases"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelSnapshotGuardObject(row["load_cases"], `${at}.load_cases`)).map(([key0, value0]) => [key0, parseLoadCase$a4dfa9739780c6d073fc74bcb99988246e24e6c3e1971cf9010a5e6f6c74099f(value0, `${at}.load_cases.${key0}`)])),
    loads: row["loads"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelSnapshotGuardObject(row["loads"], `${at}.loads`)).map(([key0, value0]) => [key0, parseStructuralLoad$a4dfa9739780c6d073fc74bcb99988246e24e6c3e1971cf9010a5e6f6c74099f(value0, `${at}.loads.${key0}`)])),
  };
}

export type { OptionGroup } from "../🟦️.ts";

export function parseOptionGroup(value: unknown, at = "$"): OptionGroup {
  const row = bimModelSnapshotGuardClosed(bimModelSnapshotGuardObject(value, at), at, ["name"]);
  return {
    name: bimModelSnapshotGuardString(row["name"], `${at}.name`, {"minLength": 1}),
  };
}

export interface PointLocation {
  readonly station: number;
}

export function parsePointLocation(value: unknown, at = "$"): PointLocation {
  const row = bimModelSnapshotGuardClosed(bimModelSnapshotGuardObject(value, at), at, ["station"]);
  return {
    station: bimModelSnapshotGuardNumber(row["station"], `${at}.station`, {"minimum": 0, "maximum": 1}),
  };
}

export interface Restraints {
  readonly x: boolean;
  readonly y: boolean;
  readonly z: boolean;
  readonly rx: boolean;
  readonly ry: boolean;
  readonly rz: boolean;
}

export function parseRestraints(value: unknown, at = "$"): Restraints {
  const row = bimModelSnapshotGuardClosed(bimModelSnapshotGuardObject(value, at), at, ["x", "y", "z", "rx", "ry", "rz"]);
  return {
    x: bimModelSnapshotGuardBoolean(row["x"], `${at}.x`),
    y: bimModelSnapshotGuardBoolean(row["y"], `${at}.y`),
    z: bimModelSnapshotGuardBoolean(row["z"], `${at}.z`),
    rx: bimModelSnapshotGuardBoolean(row["rx"], `${at}.rx`),
    ry: bimModelSnapshotGuardBoolean(row["ry"], `${at}.ry`),
    rz: bimModelSnapshotGuardBoolean(row["rz"], `${at}.rz`),
  };
}

export type { StructuralLoad } from "../🟦️.ts";

export function parseStructuralLoad(value: unknown, at = "$"): StructuralLoad {
  const row = bimModelSnapshotGuardClosed(bimModelSnapshotGuardObject(value, at), at, ["name", "load_case", "member", "location", "force", "moment"]);
  return {
    name: bimModelSnapshotGuardString(row["name"], `${at}.name`),
    load_case: bimModelSnapshotGuardString(row["load_case"], `${at}.load_case`),
    member: bimModelSnapshotGuardString(row["member"], `${at}.member`),
    location: parseStructuralLocation$a4dfa9739780c6d073fc74bcb99988246e24e6c3e1971cf9010a5e6f6c74099f(row["location"], `${at}.location`),
    force: parsePoint3(row["force"], `${at}.force`),
    moment: parsePoint3(row["moment"], `${at}.moment`),
  };
}

export type StructuralLocation = { readonly Point: import("../🟦️.ts").PointLocation } | { readonly Line: import("../🟦️.ts").LineLocation } | "Area";

export function parseStructuralLocation(value: unknown, at = "$"): StructuralLocation {
  return ((input0: unknown) => { if (typeof input0 === "object" && input0 !== null && !Array.isArray(input0)) { try { return ((row1) => ({ Point: parsePointLocation$a4dfa9739780c6d073fc74bcb99988246e24e6c3e1971cf9010a5e6f6c74099f(row1["Point"], `${at}.Point`) }))(bimModelSnapshotGuardClosed(bimModelSnapshotGuardObject(input0, `${at}`), `${at}`, ["Point"])); } catch (error) { if (!(error instanceof bimModelSnapshotGuardRefusal)) throw error; } } if (typeof input0 === "object" && input0 !== null && !Array.isArray(input0)) { try { return ((row1) => ({ Line: parseLineLocation$a4dfa9739780c6d073fc74bcb99988246e24e6c3e1971cf9010a5e6f6c74099f(row1["Line"], `${at}.Line`) }))(bimModelSnapshotGuardClosed(bimModelSnapshotGuardObject(input0, `${at}`), `${at}`, ["Line"])); } catch (error) { if (!(error instanceof bimModelSnapshotGuardRefusal)) throw error; } } if (typeof input0 === "string") { try { return bimModelSnapshotGuardConstant(input0, `${at}`, "Area"); } catch (error) { if (!(error instanceof bimModelSnapshotGuardRefusal)) throw error; } } return bimModelSnapshotGuardReject(`${at}`, "value matches no variant"); })(value);
}

export type { StructuralSupport } from "../🟦️.ts";

export function parseStructuralSupport(value: unknown, at = "$"): StructuralSupport {
  const row = bimModelSnapshotGuardClosed(bimModelSnapshotGuardObject(value, at), at, ["name", "member", "location", "offset", "restraints"]);
  return {
    name: bimModelSnapshotGuardString(row["name"], `${at}.name`),
    member: bimModelSnapshotGuardString(row["member"], `${at}.member`),
    location: parseStructuralLocation$a4dfa9739780c6d073fc74bcb99988246e24e6c3e1971cf9010a5e6f6c74099f(row["location"], `${at}.location`),
    offset: parsePoint3(row["offset"], `${at}.offset`),
    restraints: parseRestraints$a4dfa9739780c6d073fc74bcb99988246e24e6c3e1971cf9010a5e6f6c74099f(row["restraints"], `${at}.restraints`),
  };
}

export type { Workset } from "../🟦️.ts";

export function parseWorkset(value: unknown, at = "$"): Workset {
  const row = bimModelSnapshotGuardClosed(bimModelSnapshotGuardObject(value, at), at, ["name", "default_visible"]);
  return {
    name: bimModelSnapshotGuardString(row["name"], `${at}.name`, {"minLength": 1}),
    default_visible: bimModelSnapshotGuardBoolean(row["default_visible"], `${at}.default_visible`),
  };
}
