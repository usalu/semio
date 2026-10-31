/** 💡️ BIM model inference schema: derived elevations and wall layouts, keyed by element id. */
import { parsePoint3 as parsePoint3$a4dfa9739780c6d073fc74bcb99988246e24e6c3e1971cf9010a5e6f6c74099f, parsePropertyValue, parseStructuralLoad, parseStructuralSupport } from "../🟦️.ts";

import type { PropertyValue } from "../🟦️.ts";

export type PropertySource = "Own" | "Type" | "Default";

export type PropertyIssue = "Missing" | "KindMismatch" | "BelowMinimum" | "AboveMaximum" | "NotAllowed";

export interface PropertyFinding {
  template: string;
  set: string;
  property: string;
  issue: PropertyIssue;
}

export interface EffectivePropertyValue {
  value: PropertyValue;
  source: PropertySource;
  template?: string;
}

export interface EffectiveProperties {
  values: Record<string, Record<string, EffectivePropertyValue>>;
  findings: PropertyFinding[];
}

export interface StoreyLevel {
  elevation: number;
  top_elevation: number;
  absolute_elevation: number;
  absolute_top_elevation: number;
}

export type JoinEnd = "Start" | "End" | "Along";

export type JoinKind = "Miter" | "Butt" | "Through" | "Cross";

export interface Point2 {
  x: number;
  y: number;
}

export interface Vertex {
  point: Point2;
  bulge: number;
}

export interface FaceCurve {
  start: Point2;
  end: Point2;
  bulge: number;
}

export interface WallJoin {
  kind: JoinKind;
  end: JoinEnd;
  other: string;
  other_end: JoinEnd;
  point: Point2;
  overlap_area: number;
}

export interface WallLayout {
  base_z: number;
  top_z: number;
  height: number;
  thickness: number;
  length: number;
  offset_left: number;
  offset_right: number;
  layer_offsets: number[];
  left_face: FaceCurve;
  right_face: FaceCurve;
  left_length: number;
  right_length: number;
  left_area: number;
  right_area: number;
  side_area: number;
  footprint: Vertex[];
  footprint_area: number;
  volume: number;
  joins: WallJoin[];
}

export type CurtainPanel = "Glass" | "Empty" | { Solid: { material: string } } | { Door: { door_type: string } } | { Window: { window_type: string } };

export interface CellPanel {
  u: number;
  v: number;
  panel: CurtainPanel;
  id: string;
}

export interface CurtainLayout {
  base_z: number;
  top_z: number;
  height: number;
  length: number;
  area: number;
  u_panels: number;
  v_panels: number;
  u_edges: number[];
  v_edges: number[];
  panel?: CurtainPanel;
  overrides: CellPanel[];
  stray: string[];
  repeated: string[];
  ignored_u: number[];
  ignored_v: number[];
}

export type Swing = "Left" | "Right";

export type OpeningIssue = "HostMissing" | "TypeMissing" | "NonPositiveSize" | "HostDegenerate" | "OutsideHostExtent" | "BelowHostBase" | "AboveHostTop" | "OutsideTrimmedExtent" | "OverlapsSibling";

export type PlanRole = "Leaf" | "Swing" | "Glazing";

export interface Vec3 {
  x: number;
  y: number;
  z: number;
}

export interface Frame {
  origin: Vec3;
  x_axis: Vec3;
  y_axis: Vec3;
  z_axis: Vec3;
}

export interface OpeningCut {
  s_min: number;
  s_max: number;
  z_min: number;
  z_max: number;
}

export type PlanShape =
  | { Line: { from: Point2; to: Point2 } }
  | { Arc: { centre: Point2; radius: number; start_angle: number; sweep: number } };

export interface PlanStroke {
  role: PlanRole;
  shape: PlanShape;
}

export interface OpeningFrame {
  width: number;
  height: number;
  sill: number;
  offset: number;
  cut: OpeningCut;
  reveal_depth: number;
  face_front: number;
  face_back: number;
  host_length: number;
  host_height: number;
  point: Point2;
  local: Frame;
  world: Frame;
  hand?: Swing;
  plan: PlanStroke[];
  issues: OpeningIssue[];
  overlaps: string[];
  valid: boolean;
}

export interface StairRun {
  base_z: number;
  top_z: number;
  rise: number;
  riser_count: number;
  riser_height: number;
  tread_count: number;
  tread: number;
  stride: number;
  width: number;
  run_length: number;
  flights: StairFlightRun[];
  landings: StairLanding[];
  compliance: StairCompliance;
}

export interface StairFlightRun {
  first_riser: number;
  risers: number;
  treads: number;
  start: Point2;
  direction: number;
  tread: number;
  base_z: number;
  length: number;
  winder?: StairWinder;
}

export interface StairWinder {
  centre: Point2;
  inner_radius: number;
  outer_radius: number;
  start_angle: number;
  sweep: number;
}

export interface StairLanding {
  after_flight: number;
  z: number;
  centre: Point2;
  direction: number;
  width: number;
  depth: number;
}

export interface StairCompliance {
  rise_positive: boolean;
  riser_ok: boolean;
  tread_ok: boolean;
  blondel_ok: boolean;
  compliant: boolean;
}

export interface RampRun {
  base_z: number;
  top_z: number;
  rise: number;
  length: number;
  run_length: number;
  slope: number;
  angle: number;
  width: number;
  flights: RampFlight[];
  landings: RampLanding[];
  compliance: RampCompliance;
}

export interface RampFlight {
  from: number;
  to: number;
  length: number;
  z_from: number;
  z_to: number;
}

export interface RampLanding {
  from: number;
  to: number;
  length: number;
  z: number;
}

export interface RampCompliance {
  run_ok: boolean;
  slope_ok: boolean;
  compliant: boolean;
}

export interface SpaceRoom {
  status: SpaceStatus;
  outline: Vertex[];
  holes: Vertex[][];
  point: Point2;
  area: number;
  perimeter: number;
  net_floor_area: number;
  floor_z: number;
  clear_height: number;
  volume: number;
  ceiling_slab: string;
  ceiling: string;
  bounding_walls: string[];
}

export type SpaceStatus = "Inferred" | "Explicit" | "NotEnclosed" | "SeedInsideWall" | "InvalidOutline";

export interface ModelQuantities {
  elements: Record<string, ElementQuantity>;
  storeys: Record<string, QuantityTotals>;
  buildings: Record<string, QuantityTotals>;
  project: QuantityTotals;
}

export interface PanelQuantity {
  kind: string;
  count: number;
  area: number;
}

export interface MullionQuantity {
  kind: string;
  count: number;
  length: number;
}

export interface ElementQuantity {
  kind: QuantityKind;
  storey: string;
  type_id: string;
  count: number;
  length: number;
  width: number;
  height: number;
  perimeter: number;
  gross_side_area: number;
  opening_area: number;
  net_side_area: number;
  gross_area: number;
  net_area: number;
  surface_area: number;
  gross_volume: number;
  net_volume: number;
  mass: number;
  risers: number;
  balusters?: number;
  layers: LayerQuantity[];
  finishes: FinishQuantity[];
  panels?: PanelQuantity[];
  mullions?: MullionQuantity[];
  groups?: string[];
}

export type QuantityKind = "Wall" | "CurtainWall" | "Slab" | "Roof" | "Column" | "Beam" | "Window" | "Door" | "Void" | "Stair" | "Railing" | "Ramp" | "Space" | "Ceiling" | "Component" | "Mep";

export type FinishSurface = "Floor" | "Wall" | "Ceiling";

export interface FinishQuantity {
  surface: FinishSurface;
  material: string;
  area: number;
}

export interface LayerQuantity {
  material: string;
  thickness: number;
  area: number;
  volume: number;
  mass: number;
}

export interface QuantityTotals {
  kinds: Record<string, Totals>;
  types: Record<string, Totals>;
  materials: Record<string, Totals>;
  finishes: Record<string, Totals>;
  groups?: Record<string, Totals>;
}

export interface ZoneTotals {
  spaces: number;
  resolved: number;
  area: number;
  net_area: number;
  volume: number;
  occupancy: number;
  floor_finish_area: number;
  wall_finish_area: number;
  ceiling_finish_area: number;
}

export interface SchemeTotals {
  spaces: number;
  resolved: number;
  area: number;
  volume: number;
  occupancy: number;
}

export interface Totals {
  count: number;
  length: number;
  area: number;
  volume: number;
  mass: number;
}

export type SolidFamily = "Wall" | "CurtainWall" | "Window" | "Door" | "Column" | "Beam" | "Slab" | "Roof" | "Stair" | "Ramp" | "Railing" | "Ceiling" | "Component" | "Mep";

export interface SolidPoint {
  x: number;
  y: number;
  z: number;
}

export interface SolidBounds {
  min: SolidPoint;
  max: SolidPoint;
}

export interface SolidPlacement {
  x: number;
  y: number;
  z: number;
  rotation: number;
}

export interface SolidGroup {
  part: string;
  material: string;
  layer: number;
}

export interface ElementSolid {
  family: SolidFamily;
  storey: string;
  placement: SolidPlacement;
  groups: SolidGroup[];
  positions: number[];
  normals: number[];
  indices: number[];
  face_groups: number[];
  bounds: SolidBounds;
  volume: number;
  area: number;
}

export type PlanStyle = "Cut" | "Projection" | "Hidden" | "Annotation";

export type PlanKind = "WallCut" | "WallLayer" | "WallOutline" | "CurtainAxis" | "CurtainMullion" | "WindowFrame" | "WindowGlazing" | "WindowSill" | "DoorLeaf" | "DoorSwing" | "ColumnCut" | "ColumnOutline" | "BeamOutline" | "SlabEdge" | "SlabHole" | "RoofOutline" | "StairOutline" | "StairRiser" | "StairCutLine" | "StairArrow" | "StairLanding" | "RailingPath" | "SpaceOutline" | "SpaceTag" | "GridLine" | "GridBubble" | "GridLabel" | "SectionCut" | "Silhouette" | "Edge" | "Datum" | "DatumLabel" | "CeilingEdge" | "CeilingHole" | "RampOutline" | "RampLanding" | "RampArrow" | "RampTag" | "ComponentOutline" | "ComponentFront" | "ComponentConnector" | "MepAxis" | "MepBand" | "MepDrop";

export interface PlanVertex {
  x: number;
  y: number;
  bulge: number;
}

export interface PlanRegion {
  id: string;
  element: string;
  kind: PlanKind;
  style: PlanStyle;
  outer: PlanVertex[];
  holes: PlanVertex[][];
}

export interface PlanPolyline {
  id: string;
  element: string;
  kind: PlanKind;
  style: PlanStyle;
  closed: boolean;
  vertices: PlanVertex[];
}

export interface PlanText {
  id: string;
  element: string;
  kind: PlanKind;
  style: PlanStyle;
  x: number;
  y: number;
  rotation: number;
  label: string;
  detail: string;
  measure?: number;
}

export interface PlanBounds {
  min_x: number;
  min_y: number;
  max_x: number;
  max_y: number;
}

export interface PlanLinework {
  storey: string;
  cut_height: number;
  cut_elevation: number;
  regions: PlanRegion[];
  polylines: PlanPolyline[];
  texts: PlanText[];
  bounds: PlanBounds;
}

export type Severity = "Info" | "Warning" | "Error";

export type DiagnosticCode = "ClashWallWall" | "ClashWallColumn" | "ClashColumnColumn" | "ClashWallBeam" | "ClashBeamColumn" | "ClashBeamBeam" | "ClashBeamSlab" | "ClashStairWall" | "ClashStairColumn" | "ClashStairBeam" | "ClashStairStair" | "ClashSlabSlab" | "RefWallType" | "RefColumnType" | "RefBeamType" | "RefSlabType" | "RefRoofType" | "RefWindowType" | "RefDoorType" | "RefTopStorey" | "RefOpeningHost" | "RefElementStorey" | "RefStoreyBuilding" | "RefBuildingSite" | "RefGridBuilding" | "RefLayerMaterial" | "RefTypeMaterial" | "RefPropertyElement" | "DuplicateId" | "OpeningOutsideHost" | "OpeningBelowBase" | "OpeningAboveTop" | "OpeningOverlap" | "OpeningSize" | "OpeningOutsideTrimmed" | "DegenerateAxis" | "DegenerateThickness" | "DegenerateHeight" | "DegenerateProfile" | "DegenerateLoop" | "SelfIntersectingLoop" | "DegeneratePath" | "NonFinite" | "DegenerateSpacing" | "DegenerateStorey" | "StoreyLevelGap" | "StoreyLevelDuplicate" | "StoreyNoDatum" | "StairNoRise" | "StairRiserHeight" | "StairTreadDepth" | "StairComfort" | "StairStringerIgnored" | "SpaceNotEnclosed" | "SpaceSeedInWall" | "SpaceDuplicateNumber" | "RoofFlatCurved" | "RoofFlatSkeleton" | "RoofFlatDegenerate" | "RoofFlatPitch" | "RoofOverhangCollapsed" | "RoofGableToHip" | "AnnotationAnchorMissing" | "AnnotationAnchorUnresolved" | "AnnotationStyleMissing" | "DimensionZero" | "DimensionLockViolated" | "TagEmpty" | "ClashBeamCeiling" | "ClashCeilingCeiling" | "RefCeilingType" | "CurtainOverrideOutOfGrid" | "CurtainDoorNotAtBase" | "CurtainGridLineOutside" | "CurtainDuplicateOverride" | "RefCurtainWallType" | "RefCurtainPanel" | "RefCurtainOverrideHost" | "ColumnTiltInvalid" | "CeilingOutsideStorey" | "RampSlope" | "RampNoRun" | "RefRailingHost" | "RailingHostUnresolved" | "FamilySyntax" | "FamilyKind" | "FamilyCycle" | "FamilyUnknown" | "FamilyDivisionByZero" | "FamilyNegative" | "FamilyDependency" | "FamilyDomain" | "FamilyOutline" | "RefProfileFamily" | "PropertyRequiredMissing" | "PropertyKindMismatch" | "PropertyOutOfRange" | "PropertyNotAllowed" | "ClassificationUnknownCode" | "RefClassificationSystem" | "ComponentOutsideStorey" | "ComponentInWall" | "RefComponentFamily" | "RefComponentHost" | "ComponentOverride" | "MepDegenerate" | "MepClash" | "TerminalUnconnected";

export interface SeverityCounts {
  error: number;
  warning: number;
  info: number;
}

export interface ElementFindings {
  severity: Severity;
  count: number;
  codes: DiagnosticCode[];
}

export interface DiagnosticIndex {
  total: SeverityCounts;
  elements: Record<string, ElementFindings>;
  categories: Record<string, SeverityCounts>;
  codes: Record<string, number>;
  storeys: Record<string, SeverityCounts>;
}

export interface Diagnostic {
  code: DiagnosticCode;
  severity: Severity;
  message_key: string;
  elements: string[];
  missing: string[];
  storey?: string;
  values: Record<string, number>;
}

export type FamilyCategory = "Furniture" | "Equipment" | "Casework" | "Plumbing" | "Lighting" | "Mechanical" | "Electrical" | "Generic" | "Profile";

export type ParameterKind = "Length" | "Angle" | "Real" | "Integer" | "Boolean" | "Text" | "Material";

export type IssueOwner = "Parameter" | "Solid" | "Family";

export type FamilyIssueCode = "Syntax" | "Kind" | "Cycle" | "Unknown" | "DivisionByZero" | "Negative" | "Dependency" | "Domain" | "Outline";

export interface FamilyIssue {
  code: FamilyIssueCode;
  owner: IssueOwner;
  subject: string;
  field: string;
  path: number[];
  detail: string;
  names: string[];
}

export type ParameterValue = { Number: { value: number } } | { Length: { value: number } } | { Angle: { value: number } } | { Boolean: { value: boolean } } | { Text: { value: string } };

export interface ResolvedParameter {
  kind: ParameterKind;
  formula: string;
  value?: ParameterValue;
}

export interface FamilySolidMesh {
  name: string;
  material: string;
  visible: boolean;
  positions: number[];
  normals: number[];
  indices: number[];
  bounds: SolidBounds;
  volume: number;
  area: number;
}

export interface FamilyValue {
  name: string;
  category?: FamilyCategory;
  order: string[];
  parameters: Record<string, ResolvedParameter>;
  solids: Record<string, FamilySolidMesh>;
  outline: Vertex[];
  issues: FamilyIssue[];
}

export type MepSystem = "Supply" | "Return" | "Exhaust" | "DomesticWater" | "Waste" | "Gas" | "Power" | "Data" | "Lighting";

export type MepSectionKind = "Duct" | "Pipe" | "Tray";

export type MepIssueCode = "NonFinite" | "SectionDegenerate" | "PathDegenerate";

export type ComponentIssueCode = "FamilyMissing" | "FamilyProfile" | "HostMissing" | "HostOtherStorey" | "HostDegenerate" | "Override" | "NonFinite";

export interface Point3 {
  x: number;
  y: number;
  z: number;
}

export interface HostFit {
  wall: string;
  station: number;
  side: number;
  face: Point2;
  normal: Point2;
}

export interface ComponentPlacement {
  x: number;
  y: number;
  z: number;
  yaw: number;
  mirrored: boolean;
  host?: HostFit;
}

export interface Connector {
  system: MepSystem;
  colour: string;
  position: Point3;
}

export interface ComponentIssue {
  code: ComponentIssueCode;
  subject: string;
  detail: string;
  family_issue?: FamilyIssue;
}

export interface ComponentValue {
  storey: string;
  family: string;
  category?: FamilyCategory;
  placement: ComponentPlacement;
  footprint: Point2[];
  footprint_area: number;
  bounds: SolidBounds;
  volume: number;
  connector?: Connector;
  overridden: string[];
  parameters: Record<string, ResolvedParameter>;
  issues: ComponentIssue[];
}

export interface MepSection {
  kind: MepSectionKind;
  width: number;
  height: number;
  area: number;
  perimeter: number;
  label: string;
}

export interface MepSegment {
  from: Point3;
  to: Point3;
  length: number;
}

export interface MepIssue {
  code: MepIssueCode;
  detail: string;
}

export interface MepValue {
  storey: string;
  system: MepSystem;
  colour: string;
  section: MepSection;
  path: Point3[];
  segments: MepSegment[];
  length: number;
  volume: number;
  surface_area: number;
  bounds: SolidBounds;
  issues: MepIssue[];
}

export interface ModelInference {
  /** 🔬️ @derived */
  element_solids: Record<string, ElementSolid>;
  /** 🪜️ @derived */
  stair_runs: Record<string, StairRun>;
  /** ♿️ @derived */
  ramp_runs: Record<string, RampRun>;
  families: Record<string, FamilyValue>;
  components: Record<string, ComponentValue>;
  mep: Record<string, MepValue>;
  /** 🛋️ @derived */
  spaces: Record<string, SpaceRoom>;
  /** 🧮️ @derived */
  quantities: ModelQuantities;
  /** 🕳️ @derived */
  opening_frames: Record<string, OpeningFrame>;
  /** 🏢️ @derived */
  storey_levels: Record<string, StoreyLevel>;
  /** 🧱️ @derived */
  wall_layout: Record<string, WallLayout>;
  /** 🎭️ @derived */
  curtain_layout: Record<string, CurtainLayout>;
  /** 🗓️ @derived */
  plan_linework: Record<string, PlanLinework>;
  /** 🩺️ @derived */
  diagnostics: Diagnostic[];
  /** 🛏️ @derived */
  zone_totals: Record<string, ZoneTotals>;
  /** 🗺️ @derived */
  scheme_totals: Record<string, SchemeTotals>;
  /** 🔍️ @derived */
  diagnostic_index: DiagnosticIndex;
  /** 🏷️ @derived */
  effective_properties: Record<string, EffectiveProperties>;
}

import type {StructuralSupport,StructuralLoad} from "../🟦️.ts";
export interface AnalyticalMember { element:string; storey:string; kind:string; path:Point3[]; boundary:Point3[]; holes:Point3[][]; length:number; area:number; start_offset:number; end_offset:number; }
export interface RigidLink { member:string; other:string; start:Point3; end:Point3; length:number; }
export interface ResolvedSupport { authored:StructuralSupport; points:Point3[]; }
export interface ResolvedLoad { authored:StructuralLoad; points:Point3[]; factor:number; }
export interface StructuralAnalysis { members:Record<string,AnalyticalMember>; rigid_links:Record<string,RigidLink>; supports:Record<string,ResolvedSupport>; loads:Record<string,ResolvedLoad>; findings:string[]; }
export interface ModelInference { analytical_members:Record<string,AnalyticalMember>; structural_analysis:StructuralAnalysis; }

//#region 🚪️Parsers
/** 🛑️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class bimModelInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const bimModelInferenceGuardReject = (at: string, why: string): never => {
  throw new bimModelInferenceGuardRefusal(at, why);
};

type bimModelInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type bimModelInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number; readonly exclusiveMinimum?: number; readonly exclusiveMaximum?: number };
type bimModelInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const bimModelInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : bimModelInferenceGuardReject(at, "value is not an object");
export const bimModelInferenceGuardClosed = (row: Readonly<Record<string, unknown>>, at: string, keys: readonly string[]): Readonly<Record<string, unknown>> => {
  for (const key of Object.keys(row)) if (!keys.includes(key)) bimModelInferenceGuardReject(`${at}.${key}`, "unknown property");
  return row;
};
export const bimModelInferenceGuardExactObject = (value: unknown, at: string, fields: readonly string[], required: readonly string[] = fields): Readonly<Record<string, unknown>> => {
  const row = bimModelInferenceGuardClosed(bimModelInferenceGuardObject(value, at), at, fields);
  for (const field of required) if (!Object.hasOwn(row, field)) bimModelInferenceGuardReject(`${at}.${field}`, "field is missing");
  return row;
};
export const bimModelInferenceGuardArray = (value: unknown, at: string, bounds: bimModelInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return bimModelInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) bimModelInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) bimModelInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const bimModelInferenceGuardString = (value: unknown, at: string, bounds: bimModelInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return bimModelInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) bimModelInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) bimModelInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) bimModelInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const bimModelInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : bimModelInferenceGuardReject(at, "value is not a boolean"));
export const bimModelInferenceGuardNumber = (value: unknown, at: string, bounds: bimModelInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return bimModelInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) bimModelInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) bimModelInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  if (bounds.exclusiveMinimum !== undefined && value <= bounds.exclusiveMinimum) bimModelInferenceGuardReject(at, `number is not above ${bounds.exclusiveMinimum}`);
  if (bounds.exclusiveMaximum !== undefined && value >= bounds.exclusiveMaximum) bimModelInferenceGuardReject(at, `number is not below ${bounds.exclusiveMaximum}`);
  return value;
};
export const bimModelInferenceGuardInteger = (value: unknown, at: string, bounds: bimModelInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? bimModelInferenceGuardNumber(value, at, bounds) : bimModelInferenceGuardReject(at, "value is not an integer");
export const bimModelInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : bimModelInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const bimModelInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : bimModelInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseAnalyticalMember(value: unknown, at = "$"): AnalyticalMember {
  const row = bimModelInferenceGuardClosed(bimModelInferenceGuardObject(value, at), at, ["element", "storey", "kind", "path", "boundary", "holes", "length", "area", "start_offset", "end_offset"]);
  return {
    element: bimModelInferenceGuardString(row["element"], `${at}.element`),
    storey: bimModelInferenceGuardString(row["storey"], `${at}.storey`),
    kind: bimModelInferenceGuardString(row["kind"], `${at}.kind`),
    path: bimModelInferenceGuardArray(row["path"], `${at}.path`).map((item, index) => parsePoint3$a4dfa9739780c6d073fc74bcb99988246e24e6c3e1971cf9010a5e6f6c74099f(item, `${at}.path[${index}]`)),
    boundary: bimModelInferenceGuardArray(row["boundary"], `${at}.boundary`).map((item, index) => parsePoint3$a4dfa9739780c6d073fc74bcb99988246e24e6c3e1971cf9010a5e6f6c74099f(item, `${at}.boundary[${index}]`)),
    holes: bimModelInferenceGuardArray(row["holes"], `${at}.holes`).map((item, index) => bimModelInferenceGuardArray(item, `${at}.holes[${index}]`).map((item1, index1) => parsePoint3$a4dfa9739780c6d073fc74bcb99988246e24e6c3e1971cf9010a5e6f6c74099f(item1, `${at}.holes[${index}][${index1}]`))),
    length: bimModelInferenceGuardNumber(row["length"], `${at}.length`),
    area: bimModelInferenceGuardNumber(row["area"], `${at}.area`),
    start_offset: bimModelInferenceGuardNumber(row["start_offset"], `${at}.start_offset`),
    end_offset: bimModelInferenceGuardNumber(row["end_offset"], `${at}.end_offset`),
  };
}

export function parseCellPanel(value: unknown, at = "$"): CellPanel {
  const row = bimModelInferenceGuardClosed(bimModelInferenceGuardObject(value, at), at, ["u", "v", "panel", "id"]);
  return {
    u: bimModelInferenceGuardInteger(row["u"], `${at}.u`, {"minimum": 0}),
    v: bimModelInferenceGuardInteger(row["v"], `${at}.v`, {"minimum": 0}),
    panel: parseCurtainPanel(row["panel"], `${at}.panel`),
    id: bimModelInferenceGuardString(row["id"], `${at}.id`),
  };
}

export function parseComponentIssue(value: unknown, at = "$"): ComponentIssue {
  const row = bimModelInferenceGuardClosed(bimModelInferenceGuardObject(value, at), at, ["code", "subject", "detail", "family_issue"]);
  return {
    code: parseComponentIssueCode(row["code"], `${at}.code`),
    subject: bimModelInferenceGuardString(row["subject"], `${at}.subject`),
    detail: bimModelInferenceGuardString(row["detail"], `${at}.detail`),
    family_issue: row["family_issue"] === undefined ? undefined : parseFamilyIssue(row["family_issue"], `${at}.family_issue`),
  };
}

export function parseComponentIssueCode(value: unknown, at = "$"): ComponentIssueCode {
  return bimModelInferenceGuardMember(value, `${at}`, ["FamilyMissing", "FamilyProfile", "HostMissing", "HostOtherStorey", "HostDegenerate", "Override", "NonFinite"] as const);
}

export function parseComponentPlacement(value: unknown, at = "$"): ComponentPlacement {
  const row = bimModelInferenceGuardClosed(bimModelInferenceGuardObject(value, at), at, ["x", "y", "z", "yaw", "mirrored", "host"]);
  return {
    x: bimModelInferenceGuardNumber(row["x"], `${at}.x`),
    y: bimModelInferenceGuardNumber(row["y"], `${at}.y`),
    z: bimModelInferenceGuardNumber(row["z"], `${at}.z`),
    yaw: bimModelInferenceGuardNumber(row["yaw"], `${at}.yaw`),
    mirrored: bimModelInferenceGuardBoolean(row["mirrored"], `${at}.mirrored`),
    host: row["host"] === undefined ? undefined : parseHostFit(row["host"], `${at}.host`),
  };
}

export function parseComponentValue(value: unknown, at = "$"): ComponentValue {
  const row = bimModelInferenceGuardClosed(bimModelInferenceGuardObject(value, at), at, ["storey", "family", "category", "placement", "footprint", "footprint_area", "bounds", "volume", "connector", "overridden", "parameters", "issues"]);
  return {
    storey: bimModelInferenceGuardString(row["storey"], `${at}.storey`),
    family: bimModelInferenceGuardString(row["family"], `${at}.family`),
    category: row["category"] === undefined ? undefined : parseFamilyCategory(row["category"], `${at}.category`),
    placement: parseComponentPlacement(row["placement"], `${at}.placement`),
    footprint: bimModelInferenceGuardArray(row["footprint"], `${at}.footprint`).map((item, index) => parsePoint2(item, `${at}.footprint[${index}]`)),
    footprint_area: bimModelInferenceGuardNumber(row["footprint_area"], `${at}.footprint_area`),
    bounds: parseSolidBounds(row["bounds"], `${at}.bounds`),
    volume: bimModelInferenceGuardNumber(row["volume"], `${at}.volume`),
    connector: row["connector"] === undefined ? undefined : parseConnector(row["connector"], `${at}.connector`),
    overridden: bimModelInferenceGuardArray(row["overridden"], `${at}.overridden`).map((item, index) => bimModelInferenceGuardString(item, `${at}.overridden[${index}]`)),
    parameters: Object.fromEntries(Object.entries(bimModelInferenceGuardObject(row["parameters"], `${at}.parameters`)).map(([key0, value0]) => [key0, parseResolvedParameter(value0, `${at}.parameters.${key0}`)])),
    issues: bimModelInferenceGuardArray(row["issues"], `${at}.issues`).map((item, index) => parseComponentIssue(item, `${at}.issues[${index}]`)),
  };
}

export function parseConnector(value: unknown, at = "$"): Connector {
  const row = bimModelInferenceGuardClosed(bimModelInferenceGuardObject(value, at), at, ["system", "colour", "position"]);
  return {
    system: parseMepSystem(row["system"], `${at}.system`),
    colour: bimModelInferenceGuardString(row["colour"], `${at}.colour`),
    position: parsePoint3(row["position"], `${at}.position`),
  };
}

export function parseCurtainLayout(value: unknown, at = "$"): CurtainLayout {
  const row = bimModelInferenceGuardClosed(bimModelInferenceGuardObject(value, at), at, ["base_z", "top_z", "height", "length", "area", "u_panels", "v_panels", "u_edges", "v_edges", "panel", "overrides", "stray", "repeated", "ignored_u", "ignored_v"]);
  return {
    base_z: bimModelInferenceGuardNumber(row["base_z"], `${at}.base_z`),
    top_z: bimModelInferenceGuardNumber(row["top_z"], `${at}.top_z`),
    height: bimModelInferenceGuardNumber(row["height"], `${at}.height`),
    length: bimModelInferenceGuardNumber(row["length"], `${at}.length`),
    area: bimModelInferenceGuardNumber(row["area"], `${at}.area`),
    u_panels: bimModelInferenceGuardInteger(row["u_panels"], `${at}.u_panels`, {"minimum": 0}),
    v_panels: bimModelInferenceGuardInteger(row["v_panels"], `${at}.v_panels`, {"minimum": 0}),
    u_edges: bimModelInferenceGuardArray(row["u_edges"], `${at}.u_edges`).map((item, index) => bimModelInferenceGuardNumber(item, `${at}.u_edges[${index}]`)),
    v_edges: bimModelInferenceGuardArray(row["v_edges"], `${at}.v_edges`).map((item, index) => bimModelInferenceGuardNumber(item, `${at}.v_edges[${index}]`)),
    panel: row["panel"] === undefined ? undefined : parseCurtainPanel(row["panel"], `${at}.panel`),
    overrides: bimModelInferenceGuardArray(row["overrides"], `${at}.overrides`).map((item, index) => parseCellPanel(item, `${at}.overrides[${index}]`)),
    stray: bimModelInferenceGuardArray(row["stray"], `${at}.stray`).map((item, index) => bimModelInferenceGuardString(item, `${at}.stray[${index}]`)),
    repeated: bimModelInferenceGuardArray(row["repeated"], `${at}.repeated`).map((item, index) => bimModelInferenceGuardString(item, `${at}.repeated[${index}]`)),
    ignored_u: bimModelInferenceGuardArray(row["ignored_u"], `${at}.ignored_u`).map((item, index) => bimModelInferenceGuardNumber(item, `${at}.ignored_u[${index}]`)),
    ignored_v: bimModelInferenceGuardArray(row["ignored_v"], `${at}.ignored_v`).map((item, index) => bimModelInferenceGuardNumber(item, `${at}.ignored_v[${index}]`)),
  };
}

export function parseCurtainPanel(value: unknown, at = "$"): CurtainPanel {
  return ((input0: unknown) => { if (typeof input0 === "string") { try { return bimModelInferenceGuardConstant(input0, `${at}`, "Glass"); } catch (error) { if (!(error instanceof bimModelInferenceGuardRefusal)) throw error; } } if (typeof input0 === "string") { try { return bimModelInferenceGuardConstant(input0, `${at}`, "Empty"); } catch (error) { if (!(error instanceof bimModelInferenceGuardRefusal)) throw error; } } if (typeof input0 === "object" && input0 !== null && !Array.isArray(input0)) { try { return ((row1) => ({ Solid: ((row2) => ({ material: bimModelInferenceGuardString(row2["material"], `${at}.Solid.material`) }))(bimModelInferenceGuardClosed(bimModelInferenceGuardObject(row1["Solid"], `${at}.Solid`), `${at}.Solid`, ["material"])) }))(bimModelInferenceGuardClosed(bimModelInferenceGuardObject(input0, `${at}`), `${at}`, ["Solid"])); } catch (error) { if (!(error instanceof bimModelInferenceGuardRefusal)) throw error; } } if (typeof input0 === "object" && input0 !== null && !Array.isArray(input0)) { try { return ((row1) => ({ Door: ((row2) => ({ door_type: bimModelInferenceGuardString(row2["door_type"], `${at}.Door.door_type`) }))(bimModelInferenceGuardClosed(bimModelInferenceGuardObject(row1["Door"], `${at}.Door`), `${at}.Door`, ["door_type"])) }))(bimModelInferenceGuardClosed(bimModelInferenceGuardObject(input0, `${at}`), `${at}`, ["Door"])); } catch (error) { if (!(error instanceof bimModelInferenceGuardRefusal)) throw error; } } if (typeof input0 === "object" && input0 !== null && !Array.isArray(input0)) { try { return ((row1) => ({ Window: ((row2) => ({ window_type: bimModelInferenceGuardString(row2["window_type"], `${at}.Window.window_type`) }))(bimModelInferenceGuardClosed(bimModelInferenceGuardObject(row1["Window"], `${at}.Window`), `${at}.Window`, ["window_type"])) }))(bimModelInferenceGuardClosed(bimModelInferenceGuardObject(input0, `${at}`), `${at}`, ["Window"])); } catch (error) { if (!(error instanceof bimModelInferenceGuardRefusal)) throw error; } } return bimModelInferenceGuardReject(`${at}`, "value matches no variant"); })(value);
}

export function parseDiagnostic(value: unknown, at = "$"): Diagnostic {
  const row = bimModelInferenceGuardClosed(bimModelInferenceGuardObject(value, at), at, ["code", "severity", "message_key", "elements", "missing", "storey", "values"]);
  return {
    code: parseDiagnosticCode(row["code"], `${at}.code`),
    severity: parseSeverity(row["severity"], `${at}.severity`),
    message_key: bimModelInferenceGuardString(row["message_key"], `${at}.message_key`),
    elements: bimModelInferenceGuardArray(row["elements"], `${at}.elements`).map((item, index) => bimModelInferenceGuardString(item, `${at}.elements[${index}]`)),
    missing: bimModelInferenceGuardArray(row["missing"], `${at}.missing`).map((item, index) => bimModelInferenceGuardString(item, `${at}.missing[${index}]`)),
    storey: row["storey"] === undefined ? undefined : bimModelInferenceGuardString(row["storey"], `${at}.storey`),
    values: Object.fromEntries(Object.entries(bimModelInferenceGuardObject(row["values"], `${at}.values`)).map(([key0, value0]) => [key0, bimModelInferenceGuardNumber(value0, `${at}.values.${key0}`)])),
  };
}

export function parseDiagnosticCode(value: unknown, at = "$"): DiagnosticCode {
  return bimModelInferenceGuardMember(value, `${at}`, ["ClashWallWall", "ClashWallColumn", "ClashColumnColumn", "ClashWallBeam", "ClashBeamColumn", "ClashBeamBeam", "ClashBeamSlab", "ClashStairWall", "ClashStairColumn", "ClashStairBeam", "ClashStairStair", "ClashSlabSlab", "RefWallType", "RefColumnType", "RefBeamType", "RefSlabType", "RefRoofType", "RefWindowType", "RefDoorType", "RefTopStorey", "RefOpeningHost", "RefElementStorey", "RefStoreyBuilding", "RefBuildingSite", "RefGridBuilding", "RefLayerMaterial", "RefTypeMaterial", "RefPropertyElement", "DuplicateId", "OpeningOutsideHost", "OpeningBelowBase", "OpeningAboveTop", "OpeningOverlap", "OpeningSize", "OpeningOutsideTrimmed", "DegenerateAxis", "DegenerateThickness", "DegenerateHeight", "DegenerateProfile", "DegenerateLoop", "SelfIntersectingLoop", "DegeneratePath", "NonFinite", "DegenerateSpacing", "DegenerateStorey", "StoreyLevelGap", "StoreyLevelDuplicate", "StoreyNoDatum", "StairNoRise", "StairRiserHeight", "StairTreadDepth", "StairComfort", "StairStringerIgnored", "SpaceNotEnclosed", "SpaceSeedInWall", "SpaceDuplicateNumber", "RoofFlatCurved", "RoofFlatSkeleton", "RoofFlatDegenerate", "RoofFlatPitch", "RoofOverhangCollapsed", "RoofGableToHip", "AnnotationAnchorMissing", "AnnotationAnchorUnresolved", "AnnotationStyleMissing", "DimensionZero", "DimensionLockViolated", "TagEmpty", "ClashBeamCeiling", "ClashCeilingCeiling", "RefCeilingType", "CeilingOutsideStorey", "RampSlope", "RampNoRun", "RefRailingHost", "RailingHostUnresolved", "CurtainOverrideOutOfGrid", "CurtainDoorNotAtBase", "CurtainGridLineOutside", "CurtainDuplicateOverride", "RefCurtainWallType", "RefCurtainPanel", "RefCurtainOverrideHost", "ColumnTiltInvalid", "FamilySyntax", "FamilyKind", "FamilyCycle", "FamilyUnknown", "FamilyDivisionByZero", "FamilyNegative", "FamilyDependency", "FamilyDomain", "FamilyOutline", "RefProfileFamily", "PropertyRequiredMissing", "PropertyKindMismatch", "PropertyOutOfRange", "PropertyNotAllowed", "ClassificationUnknownCode", "RefClassificationSystem", "ComponentOutsideStorey", "ComponentInWall", "RefComponentFamily", "RefComponentHost", "ComponentOverride", "MepDegenerate", "MepClash", "TerminalUnconnected"] as const);
}

export function parseDiagnosticIndex(value: unknown, at = "$"): DiagnosticIndex {
  const row = bimModelInferenceGuardClosed(bimModelInferenceGuardObject(value, at), at, ["total", "elements", "categories", "codes", "storeys"]);
  return {
    total: parseSeverityCounts(row["total"], `${at}.total`),
    elements: Object.fromEntries(Object.entries(bimModelInferenceGuardObject(row["elements"], `${at}.elements`)).map(([key0, value0]) => [key0, parseElementFindings(value0, `${at}.elements.${key0}`)])),
    categories: Object.fromEntries(Object.entries(bimModelInferenceGuardObject(row["categories"], `${at}.categories`)).map(([key0, value0]) => [key0, parseSeverityCounts(value0, `${at}.categories.${key0}`)])),
    codes: Object.fromEntries(Object.entries(bimModelInferenceGuardObject(row["codes"], `${at}.codes`)).map(([key0, value0]) => [key0, bimModelInferenceGuardInteger(value0, `${at}.codes.${key0}`)])),
    storeys: Object.fromEntries(Object.entries(bimModelInferenceGuardObject(row["storeys"], `${at}.storeys`)).map(([key0, value0]) => [key0, parseSeverityCounts(value0, `${at}.storeys.${key0}`)])),
  };
}

export function parseEffectiveProperties(value: unknown, at = "$"): EffectiveProperties {
  const row = bimModelInferenceGuardClosed(bimModelInferenceGuardObject(value, at), at, ["values", "findings"]);
  return {
    values: Object.fromEntries(Object.entries(bimModelInferenceGuardObject(row["values"], `${at}.values`)).map(([key0, value0]) => [key0, Object.fromEntries(Object.entries(bimModelInferenceGuardObject(value0, `${at}.values.${key0}`)).map(([key1, value1]) => [key1, parseEffectivePropertyValue(value1, `${at}.values.${key0}.${key1}`)]))])),
    findings: bimModelInferenceGuardArray(row["findings"], `${at}.findings`).map((item, index) => parsePropertyFinding(item, `${at}.findings[${index}]`)),
  };
}

export function parseEffectivePropertyValue(value: unknown, at = "$"): EffectivePropertyValue {
  const row = bimModelInferenceGuardClosed(bimModelInferenceGuardObject(value, at), at, ["value", "source", "template"]);
  return {
    value: parsePropertyValue(row["value"], `${at}.value`),
    source: parsePropertySource(row["source"], `${at}.source`),
    template: row["template"] === undefined ? undefined : bimModelInferenceGuardString(row["template"], `${at}.template`),
  };
}

export function parseElementFindings(value: unknown, at = "$"): ElementFindings {
  const row = bimModelInferenceGuardClosed(bimModelInferenceGuardObject(value, at), at, ["severity", "count", "codes"]);
  return {
    severity: parseSeverity(row["severity"], `${at}.severity`),
    count: bimModelInferenceGuardInteger(row["count"], `${at}.count`),
    codes: bimModelInferenceGuardArray(row["codes"], `${at}.codes`).map((item, index) => parseDiagnosticCode(item, `${at}.codes[${index}]`)),
  };
}

export function parseElementQuantity(value: unknown, at = "$"): ElementQuantity {
  const row = bimModelInferenceGuardClosed(bimModelInferenceGuardObject(value, at), at, ["kind", "storey", "type_id", "count", "length", "width", "height", "perimeter", "gross_side_area", "opening_area", "net_side_area", "gross_area", "net_area", "surface_area", "gross_volume", "net_volume", "mass", "risers", "balusters", "layers", "finishes", "panels", "mullions", "groups"]);
  return {
    kind: parseQuantityKind(row["kind"], `${at}.kind`),
    storey: bimModelInferenceGuardString(row["storey"], `${at}.storey`),
    type_id: bimModelInferenceGuardString(row["type_id"], `${at}.type_id`),
    count: bimModelInferenceGuardInteger(row["count"], `${at}.count`),
    length: bimModelInferenceGuardNumber(row["length"], `${at}.length`),
    width: bimModelInferenceGuardNumber(row["width"], `${at}.width`),
    height: bimModelInferenceGuardNumber(row["height"], `${at}.height`),
    perimeter: bimModelInferenceGuardNumber(row["perimeter"], `${at}.perimeter`),
    gross_side_area: bimModelInferenceGuardNumber(row["gross_side_area"], `${at}.gross_side_area`),
    opening_area: bimModelInferenceGuardNumber(row["opening_area"], `${at}.opening_area`),
    net_side_area: bimModelInferenceGuardNumber(row["net_side_area"], `${at}.net_side_area`),
    gross_area: bimModelInferenceGuardNumber(row["gross_area"], `${at}.gross_area`),
    net_area: bimModelInferenceGuardNumber(row["net_area"], `${at}.net_area`),
    surface_area: bimModelInferenceGuardNumber(row["surface_area"], `${at}.surface_area`),
    gross_volume: bimModelInferenceGuardNumber(row["gross_volume"], `${at}.gross_volume`),
    net_volume: bimModelInferenceGuardNumber(row["net_volume"], `${at}.net_volume`),
    mass: bimModelInferenceGuardNumber(row["mass"], `${at}.mass`),
    risers: bimModelInferenceGuardInteger(row["risers"], `${at}.risers`),
    balusters: row["balusters"] === undefined ? undefined : bimModelInferenceGuardInteger(row["balusters"], `${at}.balusters`),
    layers: bimModelInferenceGuardArray(row["layers"], `${at}.layers`).map((item, index) => parseLayerQuantity(item, `${at}.layers[${index}]`)),
    finishes: bimModelInferenceGuardArray(row["finishes"], `${at}.finishes`).map((item, index) => parseFinishQuantity(item, `${at}.finishes[${index}]`)),
    panels: row["panels"] === undefined ? undefined : bimModelInferenceGuardArray(row["panels"], `${at}.panels`).map((item, index) => parsePanelQuantity(item, `${at}.panels[${index}]`)),
    mullions: row["mullions"] === undefined ? undefined : bimModelInferenceGuardArray(row["mullions"], `${at}.mullions`).map((item, index) => parseMullionQuantity(item, `${at}.mullions[${index}]`)),
    groups: row["groups"] === undefined ? undefined : bimModelInferenceGuardArray(row["groups"], `${at}.groups`).map((item, index) => bimModelInferenceGuardString(item, `${at}.groups[${index}]`)),
  };
}

export function parseElementSolid(value: unknown, at = "$"): ElementSolid {
  const row = bimModelInferenceGuardClosed(bimModelInferenceGuardObject(value, at), at, ["family", "storey", "placement", "groups", "positions", "normals", "indices", "face_groups", "bounds", "volume", "area"]);
  return {
    family: bimModelInferenceGuardMember(row["family"], `${at}.family`, ["Wall", "CurtainWall", "Window", "Door", "Column", "Beam", "Slab", "Roof", "Stair", "Railing"] as const),
    storey: bimModelInferenceGuardString(row["storey"], `${at}.storey`),
    placement: parseSolidPlacement(row["placement"], `${at}.placement`),
    groups: bimModelInferenceGuardArray(row["groups"], `${at}.groups`).map((item, index) => parseSolidGroup(item, `${at}.groups[${index}]`)),
    positions: bimModelInferenceGuardArray(row["positions"], `${at}.positions`).map((item, index) => bimModelInferenceGuardNumber(item, `${at}.positions[${index}]`)),
    normals: bimModelInferenceGuardArray(row["normals"], `${at}.normals`).map((item, index) => bimModelInferenceGuardNumber(item, `${at}.normals[${index}]`)),
    indices: bimModelInferenceGuardArray(row["indices"], `${at}.indices`).map((item, index) => bimModelInferenceGuardInteger(item, `${at}.indices[${index}]`, {"minimum": 0})),
    face_groups: bimModelInferenceGuardArray(row["face_groups"], `${at}.face_groups`).map((item, index) => bimModelInferenceGuardInteger(item, `${at}.face_groups[${index}]`, {"minimum": 0})),
    bounds: parseSolidBounds(row["bounds"], `${at}.bounds`),
    volume: bimModelInferenceGuardNumber(row["volume"], `${at}.volume`),
    area: bimModelInferenceGuardNumber(row["area"], `${at}.area`),
  };
}

export function parseFaceCurve(value: unknown, at = "$"): FaceCurve {
  const row = bimModelInferenceGuardClosed(bimModelInferenceGuardObject(value, at), at, ["start", "end", "bulge"]);
  return {
    start: parsePoint2(row["start"], `${at}.start`),
    end: parsePoint2(row["end"], `${at}.end`),
    bulge: bimModelInferenceGuardNumber(row["bulge"], `${at}.bulge`),
  };
}

export function parseFamilyCategory(value: unknown, at = "$"): FamilyCategory {
  return bimModelInferenceGuardMember(value, `${at}`, ["Furniture", "Equipment", "Casework", "Plumbing", "Lighting", "Mechanical", "Electrical", "Generic", "Profile"] as const);
}

export function parseFamilyIssue(value: unknown, at = "$"): FamilyIssue {
  const row = bimModelInferenceGuardClosed(bimModelInferenceGuardObject(value, at), at, ["code", "owner", "subject", "field", "path", "detail", "names"]);
  return {
    code: parseFamilyIssueCode(row["code"], `${at}.code`),
    owner: parseIssueOwner(row["owner"], `${at}.owner`),
    subject: bimModelInferenceGuardString(row["subject"], `${at}.subject`),
    field: bimModelInferenceGuardString(row["field"], `${at}.field`),
    path: bimModelInferenceGuardArray(row["path"], `${at}.path`).map((item, index) => bimModelInferenceGuardInteger(item, `${at}.path[${index}]`)),
    detail: bimModelInferenceGuardString(row["detail"], `${at}.detail`),
    names: bimModelInferenceGuardArray(row["names"], `${at}.names`).map((item, index) => bimModelInferenceGuardString(item, `${at}.names[${index}]`)),
  };
}

export function parseFamilyIssueCode(value: unknown, at = "$"): FamilyIssueCode {
  return bimModelInferenceGuardMember(value, `${at}`, ["Syntax", "Kind", "Cycle", "Unknown", "DivisionByZero", "Negative", "Dependency", "Domain", "Outline"] as const);
}

export function parseFamilySolidMesh(value: unknown, at = "$"): FamilySolidMesh {
  const row = bimModelInferenceGuardClosed(bimModelInferenceGuardObject(value, at), at, ["name", "material", "visible", "positions", "normals", "indices", "bounds", "volume", "area"]);
  return {
    name: bimModelInferenceGuardString(row["name"], `${at}.name`),
    material: bimModelInferenceGuardString(row["material"], `${at}.material`),
    visible: bimModelInferenceGuardBoolean(row["visible"], `${at}.visible`),
    positions: bimModelInferenceGuardArray(row["positions"], `${at}.positions`).map((item, index) => bimModelInferenceGuardNumber(item, `${at}.positions[${index}]`)),
    normals: bimModelInferenceGuardArray(row["normals"], `${at}.normals`).map((item, index) => bimModelInferenceGuardNumber(item, `${at}.normals[${index}]`)),
    indices: bimModelInferenceGuardArray(row["indices"], `${at}.indices`).map((item, index) => bimModelInferenceGuardInteger(item, `${at}.indices[${index}]`)),
    bounds: parseSolidBounds(row["bounds"], `${at}.bounds`),
    volume: bimModelInferenceGuardNumber(row["volume"], `${at}.volume`),
    area: bimModelInferenceGuardNumber(row["area"], `${at}.area`),
  };
}

export function parseFamilyValue(value: unknown, at = "$"): FamilyValue {
  const row = bimModelInferenceGuardClosed(bimModelInferenceGuardObject(value, at), at, ["name", "category", "order", "parameters", "solids", "outline", "issues"]);
  return {
    name: bimModelInferenceGuardString(row["name"], `${at}.name`),
    category: row["category"] === undefined ? undefined : parseFamilyCategory(row["category"], `${at}.category`),
    order: bimModelInferenceGuardArray(row["order"], `${at}.order`).map((item, index) => bimModelInferenceGuardString(item, `${at}.order[${index}]`)),
    parameters: Object.fromEntries(Object.entries(bimModelInferenceGuardObject(row["parameters"], `${at}.parameters`)).map(([key0, value0]) => [key0, parseResolvedParameter(value0, `${at}.parameters.${key0}`)])),
    solids: Object.fromEntries(Object.entries(bimModelInferenceGuardObject(row["solids"], `${at}.solids`)).map(([key0, value0]) => [key0, parseFamilySolidMesh(value0, `${at}.solids.${key0}`)])),
    outline: bimModelInferenceGuardArray(row["outline"], `${at}.outline`).map((item, index) => parseVertex(item, `${at}.outline[${index}]`)),
    issues: bimModelInferenceGuardArray(row["issues"], `${at}.issues`).map((item, index) => parseFamilyIssue(item, `${at}.issues[${index}]`)),
  };
}

export function parseFinishQuantity(value: unknown, at = "$"): FinishQuantity {
  const row = bimModelInferenceGuardClosed(bimModelInferenceGuardObject(value, at), at, ["surface", "material", "area"]);
  return {
    surface: parseFinishSurface(row["surface"], `${at}.surface`),
    material: bimModelInferenceGuardString(row["material"], `${at}.material`),
    area: bimModelInferenceGuardNumber(row["area"], `${at}.area`),
  };
}

export function parseFinishSurface(value: unknown, at = "$"): FinishSurface {
  return bimModelInferenceGuardMember(value, `${at}`, ["Floor", "Wall", "Ceiling"] as const);
}

export function parseFrame(value: unknown, at = "$"): Frame {
  const row = bimModelInferenceGuardClosed(bimModelInferenceGuardObject(value, at), at, ["origin", "x_axis", "y_axis", "z_axis"]);
  return {
    origin: parseVec3(row["origin"], `${at}.origin`),
    x_axis: parseVec3(row["x_axis"], `${at}.x_axis`),
    y_axis: parseVec3(row["y_axis"], `${at}.y_axis`),
    z_axis: parseVec3(row["z_axis"], `${at}.z_axis`),
  };
}

export function parseHostFit(value: unknown, at = "$"): HostFit {
  const row = bimModelInferenceGuardClosed(bimModelInferenceGuardObject(value, at), at, ["wall", "station", "side", "face", "normal"]);
  return {
    wall: bimModelInferenceGuardString(row["wall"], `${at}.wall`),
    station: bimModelInferenceGuardNumber(row["station"], `${at}.station`),
    side: bimModelInferenceGuardNumber(row["side"], `${at}.side`),
    face: parsePoint2(row["face"], `${at}.face`),
    normal: parsePoint2(row["normal"], `${at}.normal`),
  };
}

export function parseIssueOwner(value: unknown, at = "$"): IssueOwner {
  return bimModelInferenceGuardMember(value, `${at}`, ["Parameter", "Solid", "Family"] as const);
}

export function parseJoinEnd(value: unknown, at = "$"): JoinEnd {
  return bimModelInferenceGuardMember(value, `${at}`, ["Start", "End", "Along"] as const);
}

export function parseJoinKind(value: unknown, at = "$"): JoinKind {
  return bimModelInferenceGuardMember(value, `${at}`, ["Miter", "Butt", "Through", "Cross"] as const);
}

export function parseLayerQuantity(value: unknown, at = "$"): LayerQuantity {
  const row = bimModelInferenceGuardClosed(bimModelInferenceGuardObject(value, at), at, ["material", "thickness", "area", "volume", "mass"]);
  return {
    material: bimModelInferenceGuardString(row["material"], `${at}.material`),
    thickness: bimModelInferenceGuardNumber(row["thickness"], `${at}.thickness`),
    area: bimModelInferenceGuardNumber(row["area"], `${at}.area`),
    volume: bimModelInferenceGuardNumber(row["volume"], `${at}.volume`),
    mass: bimModelInferenceGuardNumber(row["mass"], `${at}.mass`),
  };
}

export function parseMepIssue(value: unknown, at = "$"): MepIssue {
  const row = bimModelInferenceGuardClosed(bimModelInferenceGuardObject(value, at), at, ["code", "detail"]);
  return {
    code: parseMepIssueCode(row["code"], `${at}.code`),
    detail: bimModelInferenceGuardString(row["detail"], `${at}.detail`),
  };
}

export function parseMepIssueCode(value: unknown, at = "$"): MepIssueCode {
  return bimModelInferenceGuardMember(value, `${at}`, ["NonFinite", "SectionDegenerate", "PathDegenerate"] as const);
}

export function parseMepSection(value: unknown, at = "$"): MepSection {
  const row = bimModelInferenceGuardClosed(bimModelInferenceGuardObject(value, at), at, ["kind", "width", "height", "area", "perimeter", "label"]);
  return {
    kind: parseMepSectionKind(row["kind"], `${at}.kind`),
    width: bimModelInferenceGuardNumber(row["width"], `${at}.width`),
    height: bimModelInferenceGuardNumber(row["height"], `${at}.height`),
    area: bimModelInferenceGuardNumber(row["area"], `${at}.area`),
    perimeter: bimModelInferenceGuardNumber(row["perimeter"], `${at}.perimeter`),
    label: bimModelInferenceGuardString(row["label"], `${at}.label`),
  };
}

export function parseMepSectionKind(value: unknown, at = "$"): MepSectionKind {
  return bimModelInferenceGuardMember(value, `${at}`, ["Duct", "Pipe", "Tray"] as const);
}

export function parseMepSegment(value: unknown, at = "$"): MepSegment {
  const row = bimModelInferenceGuardClosed(bimModelInferenceGuardObject(value, at), at, ["from", "to", "length"]);
  return {
    from: parsePoint3(row["from"], `${at}.from`),
    to: parsePoint3(row["to"], `${at}.to`),
    length: bimModelInferenceGuardNumber(row["length"], `${at}.length`),
  };
}

export function parseMepSystem(value: unknown, at = "$"): MepSystem {
  return bimModelInferenceGuardMember(value, `${at}`, ["Supply", "Return", "Exhaust", "DomesticWater", "Waste", "Gas", "Power", "Data", "Lighting"] as const);
}

export function parseMepValue(value: unknown, at = "$"): MepValue {
  const row = bimModelInferenceGuardClosed(bimModelInferenceGuardObject(value, at), at, ["storey", "system", "colour", "section", "path", "segments", "length", "volume", "surface_area", "bounds", "issues"]);
  return {
    storey: bimModelInferenceGuardString(row["storey"], `${at}.storey`),
    system: parseMepSystem(row["system"], `${at}.system`),
    colour: bimModelInferenceGuardString(row["colour"], `${at}.colour`),
    section: parseMepSection(row["section"], `${at}.section`),
    path: bimModelInferenceGuardArray(row["path"], `${at}.path`).map((item, index) => parsePoint3(item, `${at}.path[${index}]`)),
    segments: bimModelInferenceGuardArray(row["segments"], `${at}.segments`).map((item, index) => parseMepSegment(item, `${at}.segments[${index}]`)),
    length: bimModelInferenceGuardNumber(row["length"], `${at}.length`),
    volume: bimModelInferenceGuardNumber(row["volume"], `${at}.volume`),
    surface_area: bimModelInferenceGuardNumber(row["surface_area"], `${at}.surface_area`),
    bounds: parseSolidBounds(row["bounds"], `${at}.bounds`),
    issues: bimModelInferenceGuardArray(row["issues"], `${at}.issues`).map((item, index) => parseMepIssue(item, `${at}.issues[${index}]`)),
  };
}

export function parseModelInference(value: unknown, at = "$"): ModelInference {
  const row = bimModelInferenceGuardClosed(bimModelInferenceGuardObject(value, at), at, ["element_solids", "stair_runs", "ramp_runs", "spaces", "quantities", "opening_frames", "storey_levels", "wall_layout", "curtain_layout", "plan_linework", "diagnostics", "zone_totals", "scheme_totals", "diagnostic_index", "families", "effective_properties", "components", "mep", "analytical_members", "structural_analysis"]);
  return {
    element_solids: Object.fromEntries(Object.entries(bimModelInferenceGuardObject(row["element_solids"], `${at}.element_solids`)).map(([key0, value0]) => [key0, parseElementSolid(value0, `${at}.element_solids.${key0}`)])),
    stair_runs: Object.fromEntries(Object.entries(bimModelInferenceGuardObject(row["stair_runs"], `${at}.stair_runs`)).map(([key0, value0]) => [key0, parseStairRun(value0, `${at}.stair_runs.${key0}`)])),
    ramp_runs: Object.fromEntries(Object.entries(bimModelInferenceGuardObject(row["ramp_runs"], `${at}.ramp_runs`)).map(([key0, value0]) => [key0, parseRampRun(value0, `${at}.ramp_runs.${key0}`)])),
    spaces: Object.fromEntries(Object.entries(bimModelInferenceGuardObject(row["spaces"], `${at}.spaces`)).map(([key0, value0]) => [key0, parseSpaceRoom(value0, `${at}.spaces.${key0}`)])),
    quantities: parseModelQuantities(row["quantities"], `${at}.quantities`),
    opening_frames: Object.fromEntries(Object.entries(bimModelInferenceGuardObject(row["opening_frames"], `${at}.opening_frames`)).map(([key0, value0]) => [key0, parseOpeningFrame(value0, `${at}.opening_frames.${key0}`)])),
    storey_levels: Object.fromEntries(Object.entries(bimModelInferenceGuardObject(row["storey_levels"], `${at}.storey_levels`)).map(([key0, value0]) => [key0, parseStoreyLevel(value0, `${at}.storey_levels.${key0}`)])),
    wall_layout: Object.fromEntries(Object.entries(bimModelInferenceGuardObject(row["wall_layout"], `${at}.wall_layout`)).map(([key0, value0]) => [key0, parseWallLayout(value0, `${at}.wall_layout.${key0}`)])),
    curtain_layout: Object.fromEntries(Object.entries(bimModelInferenceGuardObject(row["curtain_layout"], `${at}.curtain_layout`)).map(([key0, value0]) => [key0, parseCurtainLayout(value0, `${at}.curtain_layout.${key0}`)])),
    plan_linework: Object.fromEntries(Object.entries(bimModelInferenceGuardObject(row["plan_linework"], `${at}.plan_linework`)).map(([key0, value0]) => [key0, parsePlanLinework(value0, `${at}.plan_linework.${key0}`)])),
    diagnostics: bimModelInferenceGuardArray(row["diagnostics"], `${at}.diagnostics`).map((item, index) => parseDiagnostic(item, `${at}.diagnostics[${index}]`)),
    zone_totals: Object.fromEntries(Object.entries(bimModelInferenceGuardObject(row["zone_totals"], `${at}.zone_totals`)).map(([key0, value0]) => [key0, parseZoneTotals(value0, `${at}.zone_totals.${key0}`)])),
    scheme_totals: Object.fromEntries(Object.entries(bimModelInferenceGuardObject(row["scheme_totals"], `${at}.scheme_totals`)).map(([key0, value0]) => [key0, parseSchemeTotals(value0, `${at}.scheme_totals.${key0}`)])),
    diagnostic_index: parseDiagnosticIndex(row["diagnostic_index"], `${at}.diagnostic_index`),
    families: Object.fromEntries(Object.entries(bimModelInferenceGuardObject(row["families"], `${at}.families`)).map(([key0, value0]) => [key0, parseFamilyValue(value0, `${at}.families.${key0}`)])),
    effective_properties: Object.fromEntries(Object.entries(bimModelInferenceGuardObject(row["effective_properties"], `${at}.effective_properties`)).map(([key0, value0]) => [key0, parseEffectiveProperties(value0, `${at}.effective_properties.${key0}`)])),
    components: Object.fromEntries(Object.entries(bimModelInferenceGuardObject(row["components"], `${at}.components`)).map(([key0, value0]) => [key0, parseComponentValue(value0, `${at}.components.${key0}`)])),
    mep: Object.fromEntries(Object.entries(bimModelInferenceGuardObject(row["mep"], `${at}.mep`)).map(([key0, value0]) => [key0, parseMepValue(value0, `${at}.mep.${key0}`)])),
    analytical_members: Object.fromEntries(Object.entries(bimModelInferenceGuardObject(row["analytical_members"], `${at}.analytical_members`)).map(([key0, value0]) => [key0, parseAnalyticalMember(value0, `${at}.analytical_members.${key0}`)])),
    structural_analysis: parseStructuralAnalysis(row["structural_analysis"], `${at}.structural_analysis`),
  };
}

export function parseModelQuantities(value: unknown, at = "$"): ModelQuantities {
  const row = bimModelInferenceGuardClosed(bimModelInferenceGuardObject(value, at), at, ["elements", "storeys", "buildings", "project"]);
  return {
    elements: Object.fromEntries(Object.entries(bimModelInferenceGuardObject(row["elements"], `${at}.elements`)).map(([key0, value0]) => [key0, parseElementQuantity(value0, `${at}.elements.${key0}`)])),
    storeys: Object.fromEntries(Object.entries(bimModelInferenceGuardObject(row["storeys"], `${at}.storeys`)).map(([key0, value0]) => [key0, parseQuantityTotals(value0, `${at}.storeys.${key0}`)])),
    buildings: Object.fromEntries(Object.entries(bimModelInferenceGuardObject(row["buildings"], `${at}.buildings`)).map(([key0, value0]) => [key0, parseQuantityTotals(value0, `${at}.buildings.${key0}`)])),
    project: parseQuantityTotals(row["project"], `${at}.project`),
  };
}

export function parseMullionQuantity(value: unknown, at = "$"): MullionQuantity {
  const row = bimModelInferenceGuardClosed(bimModelInferenceGuardObject(value, at), at, ["kind", "count", "length"]);
  return {
    kind: bimModelInferenceGuardString(row["kind"], `${at}.kind`),
    count: bimModelInferenceGuardInteger(row["count"], `${at}.count`, {"minimum": 0}),
    length: bimModelInferenceGuardNumber(row["length"], `${at}.length`),
  };
}

export function parseOpeningCut(value: unknown, at = "$"): OpeningCut {
  const row = bimModelInferenceGuardClosed(bimModelInferenceGuardObject(value, at), at, ["s_min", "s_max", "z_min", "z_max"]);
  return {
    s_min: bimModelInferenceGuardNumber(row["s_min"], `${at}.s_min`),
    s_max: bimModelInferenceGuardNumber(row["s_max"], `${at}.s_max`),
    z_min: bimModelInferenceGuardNumber(row["z_min"], `${at}.z_min`),
    z_max: bimModelInferenceGuardNumber(row["z_max"], `${at}.z_max`),
  };
}

export function parseOpeningFrame(value: unknown, at = "$"): OpeningFrame {
  const row = bimModelInferenceGuardClosed(bimModelInferenceGuardObject(value, at), at, ["width", "height", "sill", "offset", "cut", "reveal_depth", "face_front", "face_back", "host_length", "host_height", "point", "local", "world", "hand", "plan", "issues", "overlaps", "valid"]);
  return {
    width: bimModelInferenceGuardNumber(row["width"], `${at}.width`),
    height: bimModelInferenceGuardNumber(row["height"], `${at}.height`),
    sill: bimModelInferenceGuardNumber(row["sill"], `${at}.sill`),
    offset: bimModelInferenceGuardNumber(row["offset"], `${at}.offset`),
    cut: parseOpeningCut(row["cut"], `${at}.cut`),
    reveal_depth: bimModelInferenceGuardNumber(row["reveal_depth"], `${at}.reveal_depth`),
    face_front: bimModelInferenceGuardNumber(row["face_front"], `${at}.face_front`),
    face_back: bimModelInferenceGuardNumber(row["face_back"], `${at}.face_back`),
    host_length: bimModelInferenceGuardNumber(row["host_length"], `${at}.host_length`),
    host_height: bimModelInferenceGuardNumber(row["host_height"], `${at}.host_height`),
    point: parsePoint2(row["point"], `${at}.point`),
    local: parseFrame(row["local"], `${at}.local`),
    world: parseFrame(row["world"], `${at}.world`),
    hand: row["hand"] === undefined ? undefined : parseSwing(row["hand"], `${at}.hand`),
    plan: bimModelInferenceGuardArray(row["plan"], `${at}.plan`).map((item, index) => parsePlanStroke(item, `${at}.plan[${index}]`)),
    issues: bimModelInferenceGuardArray(row["issues"], `${at}.issues`).map((item, index) => parseOpeningIssue(item, `${at}.issues[${index}]`)),
    overlaps: bimModelInferenceGuardArray(row["overlaps"], `${at}.overlaps`).map((item, index) => bimModelInferenceGuardString(item, `${at}.overlaps[${index}]`)),
    valid: bimModelInferenceGuardBoolean(row["valid"], `${at}.valid`),
  };
}

export function parseOpeningIssue(value: unknown, at = "$"): OpeningIssue {
  return bimModelInferenceGuardMember(value, `${at}`, ["HostMissing", "TypeMissing", "NonPositiveSize", "HostDegenerate", "OutsideHostExtent", "BelowHostBase", "AboveHostTop", "OutsideTrimmedExtent", "OverlapsSibling"] as const);
}

export function parsePanelQuantity(value: unknown, at = "$"): PanelQuantity {
  const row = bimModelInferenceGuardClosed(bimModelInferenceGuardObject(value, at), at, ["kind", "count", "area"]);
  return {
    kind: bimModelInferenceGuardString(row["kind"], `${at}.kind`),
    count: bimModelInferenceGuardInteger(row["count"], `${at}.count`, {"minimum": 0}),
    area: bimModelInferenceGuardNumber(row["area"], `${at}.area`),
  };
}

export function parseParameterKind(value: unknown, at = "$"): ParameterKind {
  return bimModelInferenceGuardMember(value, `${at}`, ["Length", "Angle", "Real", "Integer", "Boolean", "Text", "Material"] as const);
}

export function parseParameterValue(value: unknown, at = "$"): ParameterValue {
  return ((row0) => { if (row0["Number"] !== undefined) { bimModelInferenceGuardClosed(row0, `${at}`, ["Number"]); return { Number: ((row1) => ({ value: bimModelInferenceGuardNumber(row1["value"], `${at}.Number.value`) }))(bimModelInferenceGuardClosed(bimModelInferenceGuardObject(row0["Number"], `${at}.Number`), `${at}.Number`, ["value"])) }; } if (row0["Length"] !== undefined) { bimModelInferenceGuardClosed(row0, `${at}`, ["Length"]); return { Length: ((row1) => ({ value: bimModelInferenceGuardNumber(row1["value"], `${at}.Length.value`) }))(bimModelInferenceGuardClosed(bimModelInferenceGuardObject(row0["Length"], `${at}.Length`), `${at}.Length`, ["value"])) }; } if (row0["Angle"] !== undefined) { bimModelInferenceGuardClosed(row0, `${at}`, ["Angle"]); return { Angle: ((row1) => ({ value: bimModelInferenceGuardNumber(row1["value"], `${at}.Angle.value`) }))(bimModelInferenceGuardClosed(bimModelInferenceGuardObject(row0["Angle"], `${at}.Angle`), `${at}.Angle`, ["value"])) }; } if (row0["Boolean"] !== undefined) { bimModelInferenceGuardClosed(row0, `${at}`, ["Boolean"]); return { Boolean: ((row1) => ({ value: bimModelInferenceGuardBoolean(row1["value"], `${at}.Boolean.value`) }))(bimModelInferenceGuardClosed(bimModelInferenceGuardObject(row0["Boolean"], `${at}.Boolean`), `${at}.Boolean`, ["value"])) }; } if (row0["Text"] !== undefined) { bimModelInferenceGuardClosed(row0, `${at}`, ["Text"]); return { Text: ((row1) => ({ value: bimModelInferenceGuardString(row1["value"], `${at}.Text.value`) }))(bimModelInferenceGuardClosed(bimModelInferenceGuardObject(row0["Text"], `${at}.Text`), `${at}.Text`, ["value"])) }; } return bimModelInferenceGuardReject(`${at}`, "value matches no variant"); })(bimModelInferenceGuardObject(value, `${at}`));
}

export function parsePlanBounds(value: unknown, at = "$"): PlanBounds {
  const row = bimModelInferenceGuardClosed(bimModelInferenceGuardObject(value, at), at, ["min_x", "min_y", "max_x", "max_y"]);
  return {
    min_x: bimModelInferenceGuardNumber(row["min_x"], `${at}.min_x`),
    min_y: bimModelInferenceGuardNumber(row["min_y"], `${at}.min_y`),
    max_x: bimModelInferenceGuardNumber(row["max_x"], `${at}.max_x`),
    max_y: bimModelInferenceGuardNumber(row["max_y"], `${at}.max_y`),
  };
}

export function parsePlanKind(value: unknown, at = "$"): PlanKind {
  return bimModelInferenceGuardMember(value, `${at}`, ["WallCut", "WallLayer", "WallOutline", "CurtainAxis", "CurtainMullion", "WindowFrame", "WindowGlazing", "WindowSill", "DoorLeaf", "DoorSwing", "ColumnCut", "ColumnOutline", "BeamOutline", "SlabEdge", "SlabHole", "RoofOutline", "StairOutline", "StairRiser", "StairCutLine", "StairArrow", "StairLanding", "RailingPath", "SpaceOutline", "SpaceTag", "GridLine", "GridBubble", "GridLabel", "SectionCut", "Silhouette", "Edge", "Datum", "DatumLabel", "CeilingEdge", "CeilingHole", "RampOutline", "RampLanding", "RampArrow", "RampTag", "ComponentOutline", "ComponentFront", "ComponentConnector", "MepAxis", "MepBand", "MepDrop"] as const);
}

export function parsePlanLinework(value: unknown, at = "$"): PlanLinework {
  const row = bimModelInferenceGuardClosed(bimModelInferenceGuardObject(value, at), at, ["storey", "cut_height", "cut_elevation", "regions", "polylines", "texts", "bounds"]);
  return {
    storey: bimModelInferenceGuardString(row["storey"], `${at}.storey`),
    cut_height: bimModelInferenceGuardNumber(row["cut_height"], `${at}.cut_height`),
    cut_elevation: bimModelInferenceGuardNumber(row["cut_elevation"], `${at}.cut_elevation`),
    regions: bimModelInferenceGuardArray(row["regions"], `${at}.regions`).map((item, index) => parsePlanRegion(item, `${at}.regions[${index}]`)),
    polylines: bimModelInferenceGuardArray(row["polylines"], `${at}.polylines`).map((item, index) => parsePlanPolyline(item, `${at}.polylines[${index}]`)),
    texts: bimModelInferenceGuardArray(row["texts"], `${at}.texts`).map((item, index) => parsePlanText(item, `${at}.texts[${index}]`)),
    bounds: parsePlanBounds(row["bounds"], `${at}.bounds`),
  };
}

export function parsePlanPolyline(value: unknown, at = "$"): PlanPolyline {
  const row = bimModelInferenceGuardClosed(bimModelInferenceGuardObject(value, at), at, ["id", "element", "kind", "style", "closed", "vertices"]);
  return {
    id: bimModelInferenceGuardString(row["id"], `${at}.id`),
    element: bimModelInferenceGuardString(row["element"], `${at}.element`),
    kind: parsePlanKind(row["kind"], `${at}.kind`),
    style: parsePlanStyle(row["style"], `${at}.style`),
    closed: bimModelInferenceGuardBoolean(row["closed"], `${at}.closed`),
    vertices: bimModelInferenceGuardArray(row["vertices"], `${at}.vertices`).map((item, index) => parsePlanVertex(item, `${at}.vertices[${index}]`)),
  };
}

export function parsePlanRegion(value: unknown, at = "$"): PlanRegion {
  const row = bimModelInferenceGuardClosed(bimModelInferenceGuardObject(value, at), at, ["id", "element", "kind", "style", "outer", "holes"]);
  return {
    id: bimModelInferenceGuardString(row["id"], `${at}.id`),
    element: bimModelInferenceGuardString(row["element"], `${at}.element`),
    kind: parsePlanKind(row["kind"], `${at}.kind`),
    style: parsePlanStyle(row["style"], `${at}.style`),
    outer: bimModelInferenceGuardArray(row["outer"], `${at}.outer`).map((item, index) => parsePlanVertex(item, `${at}.outer[${index}]`)),
    holes: bimModelInferenceGuardArray(row["holes"], `${at}.holes`).map((item, index) => bimModelInferenceGuardArray(item, `${at}.holes[${index}]`).map((item1, index1) => parsePlanVertex(item1, `${at}.holes[${index}][${index1}]`))),
  };
}

export function parsePlanRole(value: unknown, at = "$"): PlanRole {
  return bimModelInferenceGuardMember(value, `${at}`, ["Leaf", "Swing", "Glazing"] as const);
}

export function parsePlanShape(value: unknown, at = "$"): PlanShape {
  return ((row0) => { if (row0["Line"] !== undefined) { bimModelInferenceGuardClosed(row0, `${at}`, ["Line"]); return { Line: ((row1) => ({ from: parsePoint2(row1["from"], `${at}.Line.from`), to: parsePoint2(row1["to"], `${at}.Line.to`) }))(bimModelInferenceGuardClosed(bimModelInferenceGuardObject(row0["Line"], `${at}.Line`), `${at}.Line`, ["from", "to"])) }; } if (row0["Arc"] !== undefined) { bimModelInferenceGuardClosed(row0, `${at}`, ["Arc"]); return { Arc: ((row1) => ({ centre: parsePoint2(row1["centre"], `${at}.Arc.centre`), radius: bimModelInferenceGuardNumber(row1["radius"], `${at}.Arc.radius`), start_angle: bimModelInferenceGuardNumber(row1["start_angle"], `${at}.Arc.start_angle`), sweep: bimModelInferenceGuardNumber(row1["sweep"], `${at}.Arc.sweep`) }))(bimModelInferenceGuardClosed(bimModelInferenceGuardObject(row0["Arc"], `${at}.Arc`), `${at}.Arc`, ["centre", "radius", "start_angle", "sweep"])) }; } return bimModelInferenceGuardReject(`${at}`, "value matches no variant"); })(bimModelInferenceGuardObject(value, `${at}`));
}

export function parsePlanStroke(value: unknown, at = "$"): PlanStroke {
  const row = bimModelInferenceGuardClosed(bimModelInferenceGuardObject(value, at), at, ["role", "shape"]);
  return {
    role: parsePlanRole(row["role"], `${at}.role`),
    shape: parsePlanShape(row["shape"], `${at}.shape`),
  };
}

export function parsePlanStyle(value: unknown, at = "$"): PlanStyle {
  return bimModelInferenceGuardMember(value, `${at}`, ["Cut", "Projection", "Hidden", "Annotation"] as const);
}

export function parsePlanText(value: unknown, at = "$"): PlanText {
  const row = bimModelInferenceGuardClosed(bimModelInferenceGuardObject(value, at), at, ["id", "element", "kind", "style", "x", "y", "rotation", "label", "detail", "measure"]);
  return {
    id: bimModelInferenceGuardString(row["id"], `${at}.id`),
    element: bimModelInferenceGuardString(row["element"], `${at}.element`),
    kind: parsePlanKind(row["kind"], `${at}.kind`),
    style: parsePlanStyle(row["style"], `${at}.style`),
    x: bimModelInferenceGuardNumber(row["x"], `${at}.x`),
    y: bimModelInferenceGuardNumber(row["y"], `${at}.y`),
    rotation: bimModelInferenceGuardNumber(row["rotation"], `${at}.rotation`),
    label: bimModelInferenceGuardString(row["label"], `${at}.label`),
    detail: bimModelInferenceGuardString(row["detail"], `${at}.detail`),
    measure: row["measure"] === undefined ? undefined : bimModelInferenceGuardNumber(row["measure"], `${at}.measure`),
  };
}

export function parsePlanVertex(value: unknown, at = "$"): PlanVertex {
  const row = bimModelInferenceGuardClosed(bimModelInferenceGuardObject(value, at), at, ["x", "y", "bulge"]);
  return {
    x: bimModelInferenceGuardNumber(row["x"], `${at}.x`),
    y: bimModelInferenceGuardNumber(row["y"], `${at}.y`),
    bulge: bimModelInferenceGuardNumber(row["bulge"], `${at}.bulge`),
  };
}

export function parsePoint2(value: unknown, at = "$"): Point2 {
  const row = bimModelInferenceGuardClosed(bimModelInferenceGuardObject(value, at), at, ["x", "y"]);
  return {
    x: bimModelInferenceGuardNumber(row["x"], `${at}.x`),
    y: bimModelInferenceGuardNumber(row["y"], `${at}.y`),
  };
}

export function parsePoint3(value: unknown, at = "$"): Point3 {
  const row = bimModelInferenceGuardClosed(bimModelInferenceGuardObject(value, at), at, ["x", "y", "z"]);
  return {
    x: bimModelInferenceGuardNumber(row["x"], `${at}.x`),
    y: bimModelInferenceGuardNumber(row["y"], `${at}.y`),
    z: bimModelInferenceGuardNumber(row["z"], `${at}.z`),
  };
}

export function parsePropertyFinding(value: unknown, at = "$"): PropertyFinding {
  const row = bimModelInferenceGuardClosed(bimModelInferenceGuardObject(value, at), at, ["template", "set", "property", "issue"]);
  return {
    template: bimModelInferenceGuardString(row["template"], `${at}.template`),
    set: bimModelInferenceGuardString(row["set"], `${at}.set`),
    property: bimModelInferenceGuardString(row["property"], `${at}.property`),
    issue: parsePropertyIssue(row["issue"], `${at}.issue`),
  };
}

export function parsePropertyIssue(value: unknown, at = "$"): PropertyIssue {
  return bimModelInferenceGuardMember(value, `${at}`, ["Missing", "KindMismatch", "BelowMinimum", "AboveMaximum", "NotAllowed"] as const);
}

export function parsePropertySource(value: unknown, at = "$"): PropertySource {
  return bimModelInferenceGuardMember(value, `${at}`, ["Own", "Type", "Default"] as const);
}

export function parseQuantityKind(value: unknown, at = "$"): QuantityKind {
  return bimModelInferenceGuardMember(value, `${at}`, ["Wall", "CurtainWall", "Slab", "Roof", "Column", "Beam", "Window", "Door", "Void", "Stair", "Railing", "Ramp", "Space", "Ceiling", "Component", "Mep"] as const);
}

export function parseQuantityTotals(value: unknown, at = "$"): QuantityTotals {
  const row = bimModelInferenceGuardClosed(bimModelInferenceGuardObject(value, at), at, ["kinds", "types", "materials", "finishes", "groups"]);
  return {
    kinds: Object.fromEntries(Object.entries(bimModelInferenceGuardObject(row["kinds"], `${at}.kinds`)).map(([key0, value0]) => [key0, parseTotals(value0, `${at}.kinds.${key0}`)])),
    types: Object.fromEntries(Object.entries(bimModelInferenceGuardObject(row["types"], `${at}.types`)).map(([key0, value0]) => [key0, parseTotals(value0, `${at}.types.${key0}`)])),
    materials: Object.fromEntries(Object.entries(bimModelInferenceGuardObject(row["materials"], `${at}.materials`)).map(([key0, value0]) => [key0, parseTotals(value0, `${at}.materials.${key0}`)])),
    finishes: Object.fromEntries(Object.entries(bimModelInferenceGuardObject(row["finishes"], `${at}.finishes`)).map(([key0, value0]) => [key0, parseTotals(value0, `${at}.finishes.${key0}`)])),
    groups: row["groups"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelInferenceGuardObject(row["groups"], `${at}.groups`)).map(([key0, value0]) => [key0, parseTotals(value0, `${at}.groups.${key0}`)])),
  };
}

export function parseRampCompliance(value: unknown, at = "$"): RampCompliance {
  const row = bimModelInferenceGuardClosed(bimModelInferenceGuardObject(value, at), at, ["run_ok", "slope_ok", "compliant"]);
  return {
    run_ok: bimModelInferenceGuardBoolean(row["run_ok"], `${at}.run_ok`),
    slope_ok: bimModelInferenceGuardBoolean(row["slope_ok"], `${at}.slope_ok`),
    compliant: bimModelInferenceGuardBoolean(row["compliant"], `${at}.compliant`),
  };
}

export function parseRampFlight(value: unknown, at = "$"): RampFlight {
  const row = bimModelInferenceGuardClosed(bimModelInferenceGuardObject(value, at), at, ["from", "to", "length", "z_from", "z_to"]);
  return {
    from: bimModelInferenceGuardNumber(row["from"], `${at}.from`),
    to: bimModelInferenceGuardNumber(row["to"], `${at}.to`),
    length: bimModelInferenceGuardNumber(row["length"], `${at}.length`),
    z_from: bimModelInferenceGuardNumber(row["z_from"], `${at}.z_from`),
    z_to: bimModelInferenceGuardNumber(row["z_to"], `${at}.z_to`),
  };
}

export function parseRampLanding(value: unknown, at = "$"): RampLanding {
  const row = bimModelInferenceGuardClosed(bimModelInferenceGuardObject(value, at), at, ["from", "to", "length", "z"]);
  return {
    from: bimModelInferenceGuardNumber(row["from"], `${at}.from`),
    to: bimModelInferenceGuardNumber(row["to"], `${at}.to`),
    length: bimModelInferenceGuardNumber(row["length"], `${at}.length`),
    z: bimModelInferenceGuardNumber(row["z"], `${at}.z`),
  };
}

export function parseRampRun(value: unknown, at = "$"): RampRun {
  const row = bimModelInferenceGuardClosed(bimModelInferenceGuardObject(value, at), at, ["base_z", "top_z", "rise", "length", "run_length", "slope", "angle", "width", "flights", "landings", "compliance"]);
  return {
    base_z: bimModelInferenceGuardNumber(row["base_z"], `${at}.base_z`),
    top_z: bimModelInferenceGuardNumber(row["top_z"], `${at}.top_z`),
    rise: bimModelInferenceGuardNumber(row["rise"], `${at}.rise`),
    length: bimModelInferenceGuardNumber(row["length"], `${at}.length`),
    run_length: bimModelInferenceGuardNumber(row["run_length"], `${at}.run_length`),
    slope: bimModelInferenceGuardNumber(row["slope"], `${at}.slope`),
    angle: bimModelInferenceGuardNumber(row["angle"], `${at}.angle`),
    width: bimModelInferenceGuardNumber(row["width"], `${at}.width`),
    flights: bimModelInferenceGuardArray(row["flights"], `${at}.flights`).map((item, index) => parseRampFlight(item, `${at}.flights[${index}]`)),
    landings: bimModelInferenceGuardArray(row["landings"], `${at}.landings`).map((item, index) => parseRampLanding(item, `${at}.landings[${index}]`)),
    compliance: parseRampCompliance(row["compliance"], `${at}.compliance`),
  };
}

export function parseResolvedLoad(value: unknown, at = "$"): ResolvedLoad {
  const row = bimModelInferenceGuardClosed(bimModelInferenceGuardObject(value, at), at, ["authored", "points", "factor"]);
  return {
    authored: parseStructuralLoad(row["authored"], `${at}.authored`),
    points: bimModelInferenceGuardArray(row["points"], `${at}.points`).map((item, index) => parsePoint3$a4dfa9739780c6d073fc74bcb99988246e24e6c3e1971cf9010a5e6f6c74099f(item, `${at}.points[${index}]`)),
    factor: bimModelInferenceGuardNumber(row["factor"], `${at}.factor`),
  };
}

export function parseResolvedParameter(value: unknown, at = "$"): ResolvedParameter {
  const row = bimModelInferenceGuardClosed(bimModelInferenceGuardObject(value, at), at, ["kind", "formula", "value"]);
  return {
    kind: parseParameterKind(row["kind"], `${at}.kind`),
    formula: bimModelInferenceGuardString(row["formula"], `${at}.formula`),
    value: row["value"] === undefined ? undefined : parseParameterValue(row["value"], `${at}.value`),
  };
}

export function parseResolvedSupport(value: unknown, at = "$"): ResolvedSupport {
  const row = bimModelInferenceGuardClosed(bimModelInferenceGuardObject(value, at), at, ["authored", "points"]);
  return {
    authored: parseStructuralSupport(row["authored"], `${at}.authored`),
    points: bimModelInferenceGuardArray(row["points"], `${at}.points`).map((item, index) => parsePoint3$a4dfa9739780c6d073fc74bcb99988246e24e6c3e1971cf9010a5e6f6c74099f(item, `${at}.points[${index}]`)),
  };
}

export function parseRigidLink(value: unknown, at = "$"): RigidLink {
  const row = bimModelInferenceGuardClosed(bimModelInferenceGuardObject(value, at), at, ["member", "other", "start", "end", "length"]);
  return {
    member: bimModelInferenceGuardString(row["member"], `${at}.member`),
    other: bimModelInferenceGuardString(row["other"], `${at}.other`),
    start: parsePoint3$a4dfa9739780c6d073fc74bcb99988246e24e6c3e1971cf9010a5e6f6c74099f(row["start"], `${at}.start`),
    end: parsePoint3$a4dfa9739780c6d073fc74bcb99988246e24e6c3e1971cf9010a5e6f6c74099f(row["end"], `${at}.end`),
    length: bimModelInferenceGuardNumber(row["length"], `${at}.length`),
  };
}

export function parseSchemeTotals(value: unknown, at = "$"): SchemeTotals {
  const row = bimModelInferenceGuardClosed(bimModelInferenceGuardObject(value, at), at, ["spaces", "resolved", "area", "volume", "occupancy"]);
  return {
    spaces: bimModelInferenceGuardInteger(row["spaces"], `${at}.spaces`),
    resolved: bimModelInferenceGuardInteger(row["resolved"], `${at}.resolved`),
    area: bimModelInferenceGuardNumber(row["area"], `${at}.area`),
    volume: bimModelInferenceGuardNumber(row["volume"], `${at}.volume`),
    occupancy: bimModelInferenceGuardNumber(row["occupancy"], `${at}.occupancy`),
  };
}

export function parseSeverity(value: unknown, at = "$"): Severity {
  return bimModelInferenceGuardMember(value, `${at}`, ["Info", "Warning", "Error"] as const);
}

export function parseSeverityCounts(value: unknown, at = "$"): SeverityCounts {
  const row = bimModelInferenceGuardClosed(bimModelInferenceGuardObject(value, at), at, ["error", "warning", "info"]);
  return {
    error: bimModelInferenceGuardInteger(row["error"], `${at}.error`),
    warning: bimModelInferenceGuardInteger(row["warning"], `${at}.warning`),
    info: bimModelInferenceGuardInteger(row["info"], `${at}.info`),
  };
}

export function parseSolidBounds(value: unknown, at = "$"): SolidBounds {
  const row = bimModelInferenceGuardClosed(bimModelInferenceGuardObject(value, at), at, ["min", "max"]);
  return {
    min: parseSolidPoint(row["min"], `${at}.min`),
    max: parseSolidPoint(row["max"], `${at}.max`),
  };
}

export function parseSolidGroup(value: unknown, at = "$"): SolidGroup {
  const row = bimModelInferenceGuardClosed(bimModelInferenceGuardObject(value, at), at, ["part", "material", "layer"]);
  return {
    part: bimModelInferenceGuardString(row["part"], `${at}.part`),
    material: bimModelInferenceGuardString(row["material"], `${at}.material`),
    layer: bimModelInferenceGuardInteger(row["layer"], `${at}.layer`, {"minimum": 0}),
  };
}

export function parseSolidPlacement(value: unknown, at = "$"): SolidPlacement {
  const row = bimModelInferenceGuardClosed(bimModelInferenceGuardObject(value, at), at, ["x", "y", "z", "rotation"]);
  return {
    x: bimModelInferenceGuardNumber(row["x"], `${at}.x`),
    y: bimModelInferenceGuardNumber(row["y"], `${at}.y`),
    z: bimModelInferenceGuardNumber(row["z"], `${at}.z`),
    rotation: bimModelInferenceGuardNumber(row["rotation"], `${at}.rotation`),
  };
}

export function parseSolidPoint(value: unknown, at = "$"): SolidPoint {
  const row = bimModelInferenceGuardClosed(bimModelInferenceGuardObject(value, at), at, ["x", "y", "z"]);
  return {
    x: bimModelInferenceGuardNumber(row["x"], `${at}.x`),
    y: bimModelInferenceGuardNumber(row["y"], `${at}.y`),
    z: bimModelInferenceGuardNumber(row["z"], `${at}.z`),
  };
}

export function parseSpaceRoom(value: unknown, at = "$"): SpaceRoom {
  const row = bimModelInferenceGuardClosed(bimModelInferenceGuardObject(value, at), at, ["status", "outline", "holes", "point", "area", "perimeter", "net_floor_area", "floor_z", "clear_height", "volume", "ceiling_slab", "ceiling", "bounding_walls"]);
  return {
    status: parseSpaceStatus(row["status"], `${at}.status`),
    outline: bimModelInferenceGuardArray(row["outline"], `${at}.outline`).map((item, index) => parseVertex(item, `${at}.outline[${index}]`)),
    holes: bimModelInferenceGuardArray(row["holes"], `${at}.holes`).map((item, index) => bimModelInferenceGuardArray(item, `${at}.holes[${index}]`).map((item1, index1) => parseVertex(item1, `${at}.holes[${index}][${index1}]`))),
    point: parsePoint2(row["point"], `${at}.point`),
    area: bimModelInferenceGuardNumber(row["area"], `${at}.area`),
    perimeter: bimModelInferenceGuardNumber(row["perimeter"], `${at}.perimeter`),
    net_floor_area: bimModelInferenceGuardNumber(row["net_floor_area"], `${at}.net_floor_area`),
    floor_z: bimModelInferenceGuardNumber(row["floor_z"], `${at}.floor_z`),
    clear_height: bimModelInferenceGuardNumber(row["clear_height"], `${at}.clear_height`),
    volume: bimModelInferenceGuardNumber(row["volume"], `${at}.volume`),
    ceiling_slab: bimModelInferenceGuardString(row["ceiling_slab"], `${at}.ceiling_slab`),
    ceiling: bimModelInferenceGuardString(row["ceiling"], `${at}.ceiling`),
    bounding_walls: bimModelInferenceGuardArray(row["bounding_walls"], `${at}.bounding_walls`).map((item, index) => bimModelInferenceGuardString(item, `${at}.bounding_walls[${index}]`)),
  };
}

export function parseSpaceStatus(value: unknown, at = "$"): SpaceStatus {
  return bimModelInferenceGuardMember(value, `${at}`, ["Inferred", "Explicit", "NotEnclosed", "SeedInsideWall", "InvalidOutline"] as const);
}

export function parseStairCompliance(value: unknown, at = "$"): StairCompliance {
  const row = bimModelInferenceGuardClosed(bimModelInferenceGuardObject(value, at), at, ["rise_positive", "riser_ok", "tread_ok", "blondel_ok", "compliant"]);
  return {
    rise_positive: bimModelInferenceGuardBoolean(row["rise_positive"], `${at}.rise_positive`),
    riser_ok: bimModelInferenceGuardBoolean(row["riser_ok"], `${at}.riser_ok`),
    tread_ok: bimModelInferenceGuardBoolean(row["tread_ok"], `${at}.tread_ok`),
    blondel_ok: bimModelInferenceGuardBoolean(row["blondel_ok"], `${at}.blondel_ok`),
    compliant: bimModelInferenceGuardBoolean(row["compliant"], `${at}.compliant`),
  };
}

export function parseStairFlightRun(value: unknown, at = "$"): StairFlightRun {
  const row = bimModelInferenceGuardClosed(bimModelInferenceGuardObject(value, at), at, ["first_riser", "risers", "treads", "start", "direction", "tread", "base_z", "length", "winder"]);
  return {
    first_riser: bimModelInferenceGuardInteger(row["first_riser"], `${at}.first_riser`),
    risers: bimModelInferenceGuardInteger(row["risers"], `${at}.risers`),
    treads: bimModelInferenceGuardInteger(row["treads"], `${at}.treads`),
    start: parsePoint2(row["start"], `${at}.start`),
    direction: bimModelInferenceGuardNumber(row["direction"], `${at}.direction`),
    tread: bimModelInferenceGuardNumber(row["tread"], `${at}.tread`),
    base_z: bimModelInferenceGuardNumber(row["base_z"], `${at}.base_z`),
    length: bimModelInferenceGuardNumber(row["length"], `${at}.length`),
    winder: row["winder"] === undefined ? undefined : parseStairWinder(row["winder"], `${at}.winder`),
  };
}

export function parseStairLanding(value: unknown, at = "$"): StairLanding {
  const row = bimModelInferenceGuardClosed(bimModelInferenceGuardObject(value, at), at, ["after_flight", "z", "centre", "direction", "width", "depth"]);
  return {
    after_flight: bimModelInferenceGuardInteger(row["after_flight"], `${at}.after_flight`),
    z: bimModelInferenceGuardNumber(row["z"], `${at}.z`),
    centre: parsePoint2(row["centre"], `${at}.centre`),
    direction: bimModelInferenceGuardNumber(row["direction"], `${at}.direction`),
    width: bimModelInferenceGuardNumber(row["width"], `${at}.width`),
    depth: bimModelInferenceGuardNumber(row["depth"], `${at}.depth`),
  };
}

export function parseStairRun(value: unknown, at = "$"): StairRun {
  const row = bimModelInferenceGuardClosed(bimModelInferenceGuardObject(value, at), at, ["base_z", "top_z", "rise", "riser_count", "riser_height", "tread_count", "tread", "stride", "width", "run_length", "flights", "landings", "compliance"]);
  return {
    base_z: bimModelInferenceGuardNumber(row["base_z"], `${at}.base_z`),
    top_z: bimModelInferenceGuardNumber(row["top_z"], `${at}.top_z`),
    rise: bimModelInferenceGuardNumber(row["rise"], `${at}.rise`),
    riser_count: bimModelInferenceGuardInteger(row["riser_count"], `${at}.riser_count`),
    riser_height: bimModelInferenceGuardNumber(row["riser_height"], `${at}.riser_height`),
    tread_count: bimModelInferenceGuardInteger(row["tread_count"], `${at}.tread_count`),
    tread: bimModelInferenceGuardNumber(row["tread"], `${at}.tread`),
    stride: bimModelInferenceGuardNumber(row["stride"], `${at}.stride`),
    width: bimModelInferenceGuardNumber(row["width"], `${at}.width`),
    run_length: bimModelInferenceGuardNumber(row["run_length"], `${at}.run_length`),
    flights: bimModelInferenceGuardArray(row["flights"], `${at}.flights`).map((item, index) => parseStairFlightRun(item, `${at}.flights[${index}]`)),
    landings: bimModelInferenceGuardArray(row["landings"], `${at}.landings`).map((item, index) => parseStairLanding(item, `${at}.landings[${index}]`)),
    compliance: parseStairCompliance(row["compliance"], `${at}.compliance`),
  };
}

export function parseStairWinder(value: unknown, at = "$"): StairWinder {
  const row = bimModelInferenceGuardClosed(bimModelInferenceGuardObject(value, at), at, ["centre", "inner_radius", "outer_radius", "start_angle", "sweep"]);
  return {
    centre: parsePoint2(row["centre"], `${at}.centre`),
    inner_radius: bimModelInferenceGuardNumber(row["inner_radius"], `${at}.inner_radius`),
    outer_radius: bimModelInferenceGuardNumber(row["outer_radius"], `${at}.outer_radius`),
    start_angle: bimModelInferenceGuardNumber(row["start_angle"], `${at}.start_angle`),
    sweep: bimModelInferenceGuardNumber(row["sweep"], `${at}.sweep`),
  };
}

export function parseStoreyLevel(value: unknown, at = "$"): StoreyLevel {
  const row = bimModelInferenceGuardClosed(bimModelInferenceGuardObject(value, at), at, ["elevation", "top_elevation", "absolute_elevation", "absolute_top_elevation"]);
  return {
    elevation: bimModelInferenceGuardNumber(row["elevation"], `${at}.elevation`),
    top_elevation: bimModelInferenceGuardNumber(row["top_elevation"], `${at}.top_elevation`),
    absolute_elevation: bimModelInferenceGuardNumber(row["absolute_elevation"], `${at}.absolute_elevation`),
    absolute_top_elevation: bimModelInferenceGuardNumber(row["absolute_top_elevation"], `${at}.absolute_top_elevation`),
  };
}

export function parseStructuralAnalysis(value: unknown, at = "$"): StructuralAnalysis {
  const row = bimModelInferenceGuardClosed(bimModelInferenceGuardObject(value, at), at, ["members", "rigid_links", "supports", "loads", "findings"]);
  return {
    members: Object.fromEntries(Object.entries(bimModelInferenceGuardObject(row["members"], `${at}.members`)).map(([key0, value0]) => [key0, parseAnalyticalMember(value0, `${at}.members.${key0}`)])),
    rigid_links: Object.fromEntries(Object.entries(bimModelInferenceGuardObject(row["rigid_links"], `${at}.rigid_links`)).map(([key0, value0]) => [key0, parseRigidLink(value0, `${at}.rigid_links.${key0}`)])),
    supports: Object.fromEntries(Object.entries(bimModelInferenceGuardObject(row["supports"], `${at}.supports`)).map(([key0, value0]) => [key0, parseResolvedSupport(value0, `${at}.supports.${key0}`)])),
    loads: Object.fromEntries(Object.entries(bimModelInferenceGuardObject(row["loads"], `${at}.loads`)).map(([key0, value0]) => [key0, parseResolvedLoad(value0, `${at}.loads.${key0}`)])),
    findings: bimModelInferenceGuardArray(row["findings"], `${at}.findings`).map((item, index) => bimModelInferenceGuardString(item, `${at}.findings[${index}]`)),
  };
}

export function parseSwing(value: unknown, at = "$"): Swing {
  return bimModelInferenceGuardMember(value, `${at}`, ["Left", "Right"] as const);
}

export function parseTotals(value: unknown, at = "$"): Totals {
  const row = bimModelInferenceGuardClosed(bimModelInferenceGuardObject(value, at), at, ["count", "length", "area", "volume", "mass"]);
  return {
    count: bimModelInferenceGuardInteger(row["count"], `${at}.count`),
    length: bimModelInferenceGuardNumber(row["length"], `${at}.length`),
    area: bimModelInferenceGuardNumber(row["area"], `${at}.area`),
    volume: bimModelInferenceGuardNumber(row["volume"], `${at}.volume`),
    mass: bimModelInferenceGuardNumber(row["mass"], `${at}.mass`),
  };
}

export function parseVec3(value: unknown, at = "$"): Vec3 {
  const row = bimModelInferenceGuardClosed(bimModelInferenceGuardObject(value, at), at, ["x", "y", "z"]);
  return {
    x: bimModelInferenceGuardNumber(row["x"], `${at}.x`),
    y: bimModelInferenceGuardNumber(row["y"], `${at}.y`),
    z: bimModelInferenceGuardNumber(row["z"], `${at}.z`),
  };
}

export function parseVertex(value: unknown, at = "$"): Vertex {
  const row = bimModelInferenceGuardClosed(bimModelInferenceGuardObject(value, at), at, ["point", "bulge"]);
  return {
    point: parsePoint2(row["point"], `${at}.point`),
    bulge: bimModelInferenceGuardNumber(row["bulge"], `${at}.bulge`),
  };
}

export function parseWallJoin(value: unknown, at = "$"): WallJoin {
  const row = bimModelInferenceGuardClosed(bimModelInferenceGuardObject(value, at), at, ["kind", "end", "other", "other_end", "point", "overlap_area"]);
  return {
    kind: parseJoinKind(row["kind"], `${at}.kind`),
    end: parseJoinEnd(row["end"], `${at}.end`),
    other: bimModelInferenceGuardString(row["other"], `${at}.other`),
    other_end: parseJoinEnd(row["other_end"], `${at}.other_end`),
    point: parsePoint2(row["point"], `${at}.point`),
    overlap_area: bimModelInferenceGuardNumber(row["overlap_area"], `${at}.overlap_area`),
  };
}

export function parseWallLayout(value: unknown, at = "$"): WallLayout {
  const row = bimModelInferenceGuardClosed(bimModelInferenceGuardObject(value, at), at, ["base_z", "top_z", "height", "thickness", "length", "offset_left", "offset_right", "layer_offsets", "left_face", "right_face", "left_length", "right_length", "left_area", "right_area", "side_area", "footprint", "footprint_area", "volume", "joins"]);
  return {
    base_z: bimModelInferenceGuardNumber(row["base_z"], `${at}.base_z`),
    top_z: bimModelInferenceGuardNumber(row["top_z"], `${at}.top_z`),
    height: bimModelInferenceGuardNumber(row["height"], `${at}.height`),
    thickness: bimModelInferenceGuardNumber(row["thickness"], `${at}.thickness`),
    length: bimModelInferenceGuardNumber(row["length"], `${at}.length`),
    offset_left: bimModelInferenceGuardNumber(row["offset_left"], `${at}.offset_left`),
    offset_right: bimModelInferenceGuardNumber(row["offset_right"], `${at}.offset_right`),
    layer_offsets: bimModelInferenceGuardArray(row["layer_offsets"], `${at}.layer_offsets`).map((item, index) => bimModelInferenceGuardNumber(item, `${at}.layer_offsets[${index}]`)),
    left_face: parseFaceCurve(row["left_face"], `${at}.left_face`),
    right_face: parseFaceCurve(row["right_face"], `${at}.right_face`),
    left_length: bimModelInferenceGuardNumber(row["left_length"], `${at}.left_length`),
    right_length: bimModelInferenceGuardNumber(row["right_length"], `${at}.right_length`),
    left_area: bimModelInferenceGuardNumber(row["left_area"], `${at}.left_area`),
    right_area: bimModelInferenceGuardNumber(row["right_area"], `${at}.right_area`),
    side_area: bimModelInferenceGuardNumber(row["side_area"], `${at}.side_area`),
    footprint: bimModelInferenceGuardArray(row["footprint"], `${at}.footprint`).map((item, index) => parseVertex(item, `${at}.footprint[${index}]`)),
    footprint_area: bimModelInferenceGuardNumber(row["footprint_area"], `${at}.footprint_area`),
    volume: bimModelInferenceGuardNumber(row["volume"], `${at}.volume`),
    joins: bimModelInferenceGuardArray(row["joins"], `${at}.joins`).map((item, index) => parseWallJoin(item, `${at}.joins[${index}]`)),
  };
}

export function parseZoneTotals(value: unknown, at = "$"): ZoneTotals {
  const row = bimModelInferenceGuardClosed(bimModelInferenceGuardObject(value, at), at, ["spaces", "resolved", "area", "net_area", "volume", "occupancy", "floor_finish_area", "wall_finish_area", "ceiling_finish_area"]);
  return {
    spaces: bimModelInferenceGuardInteger(row["spaces"], `${at}.spaces`),
    resolved: bimModelInferenceGuardInteger(row["resolved"], `${at}.resolved`),
    area: bimModelInferenceGuardNumber(row["area"], `${at}.area`),
    net_area: bimModelInferenceGuardNumber(row["net_area"], `${at}.net_area`),
    volume: bimModelInferenceGuardNumber(row["volume"], `${at}.volume`),
    occupancy: bimModelInferenceGuardNumber(row["occupancy"], `${at}.occupancy`),
    floor_finish_area: bimModelInferenceGuardNumber(row["floor_finish_area"], `${at}.floor_finish_area`),
    wall_finish_area: bimModelInferenceGuardNumber(row["wall_finish_area"], `${at}.wall_finish_area`),
    ceiling_finish_area: bimModelInferenceGuardNumber(row["ceiling_finish_area"], `${at}.ceiling_finish_area`),
  };
}
