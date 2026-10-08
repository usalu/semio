/** 💡️ BIM model inference schema: derived elevations and wall layouts, keyed by element id. */

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

export interface CurtainLayout {
  base_z: number;
  top_z: number;
  height: number;
  length: number;
  area: number;
  u_panels: number;
  v_panels: number;
  panel_width: number;
  panel_height: number;
}

export type Swing = "Left" | "Right";

export type OpeningIssue = "HostMissing" | "TypeMissing" | "NonPositiveSize" | "OutsideHostExtent" | "BelowHostBase" | "AboveHostTop" | "OverlapsSibling";

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
  bounding_walls: string[];
}

export type SpaceStatus = "Inferred" | "Explicit" | "NotEnclosed" | "SeedInsideWall" | "InvalidOutline";

export interface ModelQuantities {
  elements: Record<string, ElementQuantity>;
  storeys: Record<string, QuantityTotals>;
  buildings: Record<string, QuantityTotals>;
  project: QuantityTotals;
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
  layers: LayerQuantity[];
}

export type QuantityKind = "Wall" | "CurtainWall" | "Slab" | "Roof" | "Column" | "Beam" | "Window" | "Door" | "Void" | "Stair" | "Railing" | "Space";

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
}

export interface Totals {
  count: number;
  length: number;
  area: number;
  volume: number;
  mass: number;
}

export type SolidFamily = "Wall" | "CurtainWall" | "Window" | "Door" | "Column" | "Beam" | "Slab" | "Roof" | "Stair" | "Railing";

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

export type PlanKind = "WallCut" | "WallLayer" | "WallOutline" | "CurtainAxis" | "CurtainMullion" | "WindowFrame" | "WindowGlazing" | "WindowSill" | "DoorLeaf" | "DoorSwing" | "ColumnCut" | "ColumnOutline" | "BeamOutline" | "SlabEdge" | "SlabHole" | "RoofOutline" | "StairOutline" | "StairRiser" | "StairCutLine" | "StairArrow" | "StairLanding" | "RailingPath" | "SpaceOutline" | "SpaceTag" | "GridLine" | "GridBubble" | "GridLabel";

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

export type DiagnosticCode = "ClashWallWall" | "ClashWallColumn" | "ClashColumnColumn" | "ClashWallBeam" | "ClashBeamColumn" | "ClashBeamBeam" | "ClashBeamSlab" | "ClashStairWall" | "ClashStairColumn" | "ClashStairBeam" | "ClashStairStair" | "ClashSlabSlab" | "RefWallType" | "RefColumnType" | "RefBeamType" | "RefSlabType" | "RefRoofType" | "RefWindowType" | "RefDoorType" | "RefTopStorey" | "RefOpeningHost" | "RefElementStorey" | "RefStoreyBuilding" | "RefBuildingSite" | "RefGridBuilding" | "RefLayerMaterial" | "RefTypeMaterial" | "RefPropertyElement" | "DuplicateId" | "OpeningOutsideHost" | "OpeningBelowBase" | "OpeningAboveTop" | "OpeningOverlap" | "OpeningSize" | "DegenerateAxis" | "DegenerateThickness" | "DegenerateHeight" | "DegenerateProfile" | "DegenerateLoop" | "SelfIntersectingLoop" | "DegeneratePath" | "NonFinite" | "DegenerateSpacing" | "DegenerateStorey" | "StoreyLevelGap" | "StoreyLevelDuplicate" | "StoreyNoDatum" | "StairNoRise" | "StairRiserHeight" | "StairTreadDepth" | "StairComfort" | "SpaceNotEnclosed" | "SpaceSeedInWall" | "SpaceDuplicateNumber";

export interface Diagnostic {
  code: DiagnosticCode;
  severity: Severity;
  message_key: string;
  elements: string[];
  missing: string[];
  storey?: string;
  values: Record<string, number>;
}

export interface ModelInference {
  /** @derived */
  element_solids: Record<string, ElementSolid>;
  /** @derived */
  stair_runs: Record<string, StairRun>;
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
}
