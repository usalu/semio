/** 💡️ BIM model inference schema: derived elevations and wall layouts, keyed by element id. */

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
  /** @derived */
  element_solids: Record<string, ElementSolid>;
  /** @derived */
  stair_runs: Record<string, StairRun>;
  /** @derived */
  ramp_runs: Record<string, RampRun>;
  families: Record<string, FamilyValue>;
  components: Record<string, ComponentValue>;
  mep: Record<string, MepValue>;
  /** @derived */
  spaces: Record<string, SpaceRoom>;
  /** @derived */
  quantities: ModelQuantities;
  /** @derived */
  opening_frames: Record<string, OpeningFrame>;
  /** @derived */
  storey_levels: Record<string, StoreyLevel>;
  /** @derived */
  wall_layout: Record<string, WallLayout>;
  /** @derived */
  curtain_layout: Record<string, CurtainLayout>;
  /** @derived */
  plan_linework: Record<string, PlanLinework>;
  /** @derived */
  diagnostics: Diagnostic[];
  /** @derived */
  zone_totals: Record<string, ZoneTotals>;
  /** @derived */
  scheme_totals: Record<string, SchemeTotals>;
  /** @derived */
  diagnostic_index: DiagnosticIndex;
  /** @derived */
  effective_properties: Record<string, EffectiveProperties>;
}

import type {StructuralSupport,StructuralLoad} from "../🟦️.ts";
export interface AnalyticalMember { element:string; storey:string; kind:string; path:Point3[]; boundary:Point3[]; holes:Point3[][]; length:number; area:number; start_offset:number; end_offset:number; }
export interface RigidLink { member:string; other:string; start:Point3; end:Point3; length:number; }
export interface ResolvedSupport { authored:StructuralSupport; points:Point3[]; }
export interface ResolvedLoad { authored:StructuralLoad; points:Point3[]; factor:number; }
export interface StructuralAnalysis { members:Record<string,AnalyticalMember>; rigid_links:Record<string,RigidLink>; supports:Record<string,ResolvedSupport>; loads:Record<string,ResolvedLoad>; findings:string[]; }
export interface ModelInference { analytical_members:Record<string,AnalyticalMember>; structural_analysis:StructuralAnalysis; }
