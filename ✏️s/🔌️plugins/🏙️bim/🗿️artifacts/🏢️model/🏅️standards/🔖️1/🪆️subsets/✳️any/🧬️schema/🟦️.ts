/** 🧬️ BIM model artifact schema — every field with its state class. */

export interface ModelArtifact {
  /** @state artifact */
  schema: string;
  /** @state artifact */
  project: Project;
  /** @state artifact */
  materials?: Record<string, Material>;
  /** @state artifact */
  wall_types?: Record<string, WallType>;
  /** @state artifact */
  slab_types?: Record<string, SlabType>;
  /** @state artifact */
  roof_types?: Record<string, RoofType>;
  /** @state artifact */
  column_types?: Record<string, ColumnType>;
  /** @state artifact */
  beam_types?: Record<string, BeamType>;
  /** @state artifact */
  window_types?: Record<string, WindowType>;
  /** @state artifact */
  door_types?: Record<string, DoorType>;
  /** @state artifact */
  curtain_wall_types?: Record<string, CurtainWallType>;
  /** @state artifact */
  sites?: Record<string, Site>;
  /** @state artifact */
  buildings?: Record<string, Building>;
  /** @state artifact */
  storeys?: Record<string, Storey>;
  /** @state artifact */
  grids?: Record<string, GridLine>;
  /** @state artifact */
  walls?: Record<string, Wall>;
  /** @state artifact */
  curtain_walls?: Record<string, CurtainWall>;
  /** @state artifact */
  curtain_panel_overrides?: Record<string, CurtainPanelOverride>;
  /** @state artifact */
  columns?: Record<string, Column>;
  /** @state artifact */
  beams?: Record<string, Beam>;
  /** @state artifact */
  slabs?: Record<string, Slab>;
  /** @state artifact */
  roofs?: Record<string, Roof>;
  /** @state artifact */
  openings?: Record<string, Opening>;
  /** @state artifact */
  stairs?: Record<string, Stair>;
  /** @state artifact */
  railings?: Record<string, Railing>;
  /** @state artifact */
  ramps?: Record<string, Ramp>;
  /** @state artifact */
  spaces?: Record<string, Space>;
  /** @state artifact */
  ceiling_types?: Record<string, CeilingType>;
  /** @state artifact */
  ceilings?: Record<string, Ceiling>;
  /** @state artifact */
  zones?: Record<string, Zone>;
  /** @state artifact */
  area_schemes?: Record<string, AreaScheme>;
  /** @state artifact */
  views?: Record<string, View>;
  /** @state artifact */
  sheets?: Record<string, Sheet>;
  /** @state artifact */
  viewports?: Record<string, Viewport>;
  /** @state artifact */
  sheet_revisions?: Record<string, SheetRevision>;
  /** @state artifact */
  dimensions?: Record<string, Dimension>;
  /** @state artifact */
  tags?: Record<string, Tag>;
  /** @state artifact */
  text_notes?: Record<string, TextNote>;
  /** @state artifact */
  leaders?: Record<string, Leader>;
  /** @state artifact */
  annotation_styles?: Record<string, AnnotationStyle>;
  /** @state artifact */
  families?: Record<string, Family>;
  /** @state artifact */
  family_parameters?: Record<string, FamilyParameter>;
  /** @state artifact */
  family_solids?: Record<string, FamilySolid>;
  /** @state artifact */
  wall_sweeps?: Record<string, WallSweep>;
  /** @state artifact */
  schedules?: Record<string, Schedule>;
  /** @state artifact */
  property_templates?: Record<string, PropertyTemplate>;
  /** @state artifact */
  classification_systems?: Record<string, ClassificationSystem>;
  /** @state artifact */
  properties?: Record<string, PropertySet>;
  /** @state artifact */
  classifications?: Record<string, ClassificationSet>;
}

export type PropertyKind = "Text" | "Real" | "Integer" | "Boolean" | "Length" | "Area" | "Volume" | "Angle";

export type TemplateTarget = "Site" | "Building" | "Storey" | "Wall" | "CurtainWall" | "Column" | "Beam" | "Slab" | "Ceiling" | "Roof" | "Window" | "Door" | "Void" | "Stair" | "Ramp" | "Railing" | "Space" | "Zone" | "WallType" | "SlabType" | "CeilingType" | "RoofType" | "ColumnType" | "BeamType" | "WindowType" | "DoorType";

export type AnchorEnd = "Start" | "End";

export type WallSide = "Left" | "Right";

export type TagCategory = "Name" | "Type" | "Number" | "Size";

export type Terminator = "Tick" | "Arrow" | "Dot";

export type DimensionUnit = "Metre" | "Centimetre" | "Millimetre";

export type FamilyCategory = "Furniture" | "Equipment" | "Casework" | "Plumbing" | "Lighting" | "Mechanical" | "Electrical" | "Generic" | "Profile";

export type ParameterKind = "Length" | "Angle" | "Real" | "Integer" | "Boolean" | "Text" | "Material";

export type SolidAxis = "X" | "Y" | "Z";

export type IsoSize = "A0" | "A1" | "A2" | "A3" | "A4";

export type Orientation = "Landscape" | "Portrait";

export type MaterialCategory = "Concrete" | "Masonry" | "Wood" | "Metal" | "Glass" | "Insulation" | "Finish" | "Membrane" | "Other";

export type LayerFunction = "Structure" | "Substrate" | "Insulation" | "Finish" | "Membrane" | "Core";

export type LocationLine = "Center" | "Interior" | "Exterior" | "CoreCenter";

export type Phase = "Existing" | "New" | "Demolished" | "Temporary";

export type DoorLeaves = "Single" | "Double";

export type Swing = "Left" | "Right";

export type Turn = "Left" | "Right";

export type StringerKind = "None" | "Closed" | "Open" | "Mono";

export type RiserKind = "Open" | "Closed";

export type ScheduleCategory = "Wall" | "CurtainWall" | "Slab" | "Roof" | "Column" | "Beam" | "Window" | "Door" | "Void" | "Stair" | "Railing" | "Space" | "Finish" | "Material";

export type ScheduleField = "Id" | "Name" | "Kind" | "Storey" | "Level" | "Type" | "Phase" | "Material" | "Host" | "Number" | "Usage" | "Surface" | "Swing" | "Leaves" | "Panes" | "Count" | "Length" | "Width" | "Height" | "Perimeter" | "GrossSideArea" | "OpeningArea" | "NetSideArea" | "GrossArea" | "NetArea" | "SurfaceArea" | "GrossVolume" | "NetVolume" | "Mass" | "Risers" | "Thickness" | "LayerArea" | "LayerVolume" | "LayerMass" | "FinishArea";

export type ScheduleOp = "Equals" | "NotEquals" | "Contains" | "Greater" | "GreaterOrEqual" | "Less" | "LessOrEqual" | "Empty" | "NotEmpty";

export type EndJoin = "Miter" | "Butt" | "None";

export type ViewKind = "Plan" | "CeilingPlan" | "Section" | "Elevation" | "Orthographic" | "Perspective";

export type DetailLevel = "Coarse" | "Medium" | "Fine";

export type ViewCategory = "Walls" | "CurtainWalls" | "Columns" | "Beams" | "Slabs" | "Roofs" | "Openings" | "Stairs" | "Railings" | "Spaces" | "Grids";

export type HostSide = "Left" | "Right";

export type AreaMeasure = "Gross" | "Net";

export type AnnotationAnchor =
  | { Point: { point: Point2 } }
  | { WallFace: { wall: string; side: WallSide } }
  | { WallAxis: { wall: string } }
  | { WallEnd: { wall: string; end: AnchorEnd } }
  | { OpeningCentre: { opening: string } }
  | { Grid: { grid: string } }
  | { ColumnCentre: { column: string } };

export type ParametricProfile =
  | { Rectangle: { width: string; depth: string } }
  | { Circle: { diameter: string } }
  | { IShape: { width: string; depth: string; web: string; flange: string } }
  | { Polygon: { points: ExprPoint[] } };

export type SolidShape =
  | { Extrusion: { profile: ParametricProfile; base: string; height: string } }
  | { Revolution: { profile: ParametricProfile; axis: SolidAxis; angle: string } }
  | { Sweep: { profile: ParametricProfile; path: ExprPoint[] } }
  | { Cuboid: { x: string; y: string; z: string; width: string; depth: string; height: string } };

export type Paper =
  | { Iso: { size: IsoSize } }
  | { Custom: { width: number; height: number } };

export type CurtainGrid =
  | { Spacing: { spacing: number } }
  | { Lines: { positions: number[] } };

export type CurtainPanel =
  | "Glass"
  | { Solid: { material: string } }
  | { Door: { door_type: string } }
  | { Window: { window_type: string } }
  | "Empty";

export type Axis =
  | { Line: { start: Point2; end: Point2 } }
  | { Arc: { start: Point2; end: Point2; bulge: number } };

export type TopConstraint =
  | { Unconnected: { height: number } }
  | { StoreyTop: { offset: number } }
  | { Storey: { storey: string; offset: number } }
  | { Roof: { roof: string; offset: number } }
  | { Slab: { slab: string; offset: number } }
  | { Ceiling: { ceiling: string; offset: number } };

export type Profile =
  | { Rectangle: { width: number; depth: number } }
  | { Circle: { diameter: number } }
  | { IShape: { width: number; depth: number; web: number; flange: number } }
  | { Custom: { outline: Vertex[] } }
  | { Family: { family: string } };

export type RoofShape =
  | "Flat"
  | { Shed: { pitch: number; direction: number } }
  | { Gable: { pitch: number; ridge_direction: number } }
  | { Hip: { pitch: number } }
  | { Mansard: { lower_pitch: number; upper_pitch: number; break_height: number } };

export type OpeningKind =
  | { Window: { window_type: string } }
  | { Door: { door_type: string } }
  | { Void: { width: number; height: number } };

export type StairFlight =
  | "Straight"
  | { LTurn: { split: number; turn: Turn } }
  | { UTurn: { gap: number } }
  | { Spiral: { radius: number; sweep: number } };

export type PropertyValue =
  | { Text: { value: string } }
  | { Real: { value: number } }
  | { Integer: { value: number } }
  | { Boolean: { value: boolean } }
  | { Length: { value: number } }
  | { Area: { value: number } }
  | { Volume: { value: number } }
  | { Angle: { value: number } };

export type Infill =
  | "None"
  | { Glass: { thickness: number } }
  | { Panel: { thickness: number } };

export type SpaceBoundary =
  | { Bounded: { seed: Point2 } }
  | { Explicit: { outline: Vertex[] } };

export type ScheduleKey =
  | { Field: { field: ScheduleField } }
  | { Property: { set: string; name: string } };

export interface PropertyDef {
  name: string;
  kind: PropertyKind;
  unit?: string;
  description?: string;
  required: boolean;
  default_value?: PropertyValue;
  allowed: PropertyValue[];
  minimum?: number;
  maximum?: number;
}

export interface PropertyTemplate {
  name: string;
  applies_to: TemplateTarget[];
  properties: PropertyDef[];
}

export interface ClassificationItem {
  code: string;
  title: string;
  parent?: string;
}

export interface ClassificationSystem {
  name: string;
  edition: string;
  source?: string;
  entries: ClassificationItem[];
}

export interface Dimension {
  storey: string;
  anchors: AnnotationAnchor[];
  angle: number;
  offset: number;
  style: string;
  lock?: number;
  name: string;
}

export interface Tag {
  storey: string;
  element: string;
  category: TagCategory;
  offset: Point2;
  style: string;
}

export interface TextNote {
  storey: string;
  position: Point2;
  text: string;
  rotation: number;
  style: string;
}

export interface Leader {
  storey: string;
  anchor: AnnotationAnchor;
  offset: Point2;
  text: string;
  style: string;
}

export interface AnnotationStyle {
  name: string;
  text_height: number;
  terminator: Terminator;
  unit: DimensionUnit;
  precision: number;
  mark_size: number;
  gap: number;
  overshoot: number;
}

export interface ExprPoint {
  x: string;
  y: string;
}

export interface ExprPoint3 {
  x: string;
  y: string;
  z: string;
}

export interface Family {
  name: string;
  category: FamilyCategory;
}

export interface FamilyParameter {
  family: string;
  name: string;
  kind: ParameterKind;
  value: string;
}

export interface FamilySolid {
  family: string;
  name: string;
  shape: SolidShape;
  material: string;
  visible: string;
  offset: ExprPoint3;
}

export interface Sheet {
  number: string;
  name: string;
  paper: Paper;
  orientation: Orientation;
  project: string;
  drawn_by: string;
  checked_by: string;
  date: string;
  revision: string;
  scale_label: string;
}

export interface Viewport {
  sheet: string;
  view: string;
  position: Point2;
  scale: number;
  crop?: ViewCrop;
  label?: string;
}

export interface SheetRevision {
  sheet: string;
  number: string;
  date: string;
  description: string;
  author: string;
}

export interface WallSweep {
  host: string;
  side: WallSide;
  profile: Profile;
  height: number;
  inset: number;
  material: string;
  name: string;
}

export interface Point2 {
  x: number;
  y: number;
}

export interface Rgb {
  r: number;
  g: number;
  b: number;
}

export interface Vertex {
  point: Point2;
  bulge: number;
}

export interface Slope {
  direction: number;
  angle: number;
}

export interface StairStringer {
  kind: StringerKind;
  width: number;
  depth: number;
}

export interface Baluster {
  profile: Profile;
  spacing: number;
}

export interface RailingHost {
  element: string;
  side: HostSide;
  edge: number;
  inset: number;
}

export interface Layer {
  material: string;
  thickness: number;
  function: LayerFunction;
}

export interface Project {
  name: string;
  description: string;
  author: string;
  organization: string;
  phase_names: string[];
}

export interface Material {
  name: string;
  category: MaterialCategory;
  color: Rgb;
  density: number;
  conductivity: number;
  specific_heat: number;
}

export interface WallType {
  name: string;
  layers: Layer[];
}

export interface SlabType {
  name: string;
  layers: Layer[];
}

export interface CeilingType {
  name: string;
  layers: Layer[];
}

export interface RoofType {
  name: string;
  layers: Layer[];
}

export interface ColumnType {
  name: string;
  profile: Profile;
  material: string;
}

export interface BeamType {
  name: string;
  profile: Profile;
  material: string;
}

export interface WindowType {
  name: string;
  width: number;
  height: number;
  sill: number;
  frame_width: number;
  frame_depth: number;
  panes: number;
  material: string;
}

export interface DoorType {
  name: string;
  width: number;
  height: number;
  frame_width: number;
  frame_depth: number;
  leaves: DoorLeaves;
  swing: Swing;
  material: string;
}

export interface Site {
  name: string;
  latitude: number;
  longitude: number;
  elevation: number;
  true_north: number;
  boundary: Point2[];
}

export interface Building {
  site: string;
  name: string;
  origin: Point2;
  rotation: number;
  elevation: number;
}

export interface Storey {
  building: string;
  name: string;
  level: number;
  height: number;
  cut_height?: number;
}

export interface GridLine {
  building: string;
  label: string;
  start: Point2;
  end: Point2;
}

export interface Wall {
  storey: string;
  wall_type: string;
  axis: Axis;
  location: LocationLine;
  base_offset: number;
  top: TopConstraint;
  phase: Phase;
  start_join?: EndJoin;
  end_join?: EndJoin;
  name: string;
  base_slab?: string;
}

export interface CurtainWallType {
  name: string;
  u_grid: CurtainGrid;
  v_grid: CurtainGrid;
  interior_mullion: Profile;
  border_mullion: Profile;
  panel: CurtainPanel;
  panel_material: string;
  mullion_material: string;
}

export interface CurtainPanelOverride {
  curtain: string;
  u: number;
  v: number;
  panel: CurtainPanel;
}

export interface CurtainWall {
  storey: string;
  curtain_wall_type: string;
  axis: Axis;
  base_offset: number;
  top: TopConstraint;
  u_grid?: CurtainGrid;
  v_grid?: CurtainGrid;
  phase: Phase;
  name: string;
}

export interface Column {
  storey: string;
  column_type: string;
  position: Point2;
  rotation: number;
  tilt?: Slope;
  base_offset: number;
  top: TopConstraint;
  phase: Phase;
  name: string;
}

export interface Beam {
  storey: string;
  beam_type: string;
  axis: Axis;
  top_offset: number;
  end_top_offset?: number;
  phase: Phase;
  name: string;
}

export interface Slab {
  storey: string;
  slab_type: string;
  boundary: Vertex[];
  holes: Vertex[][];
  offset: number;
  slope?: Slope;
  phase: Phase;
  name: string;
}

export interface Ceiling {
  storey: string;
  ceiling_type: string;
  boundary: Vertex[];
  holes: Vertex[][];
  offset: number;
  slope?: Slope;
  name: string;
}

export interface Roof {
  storey: string;
  roof_type: string;
  footprint: Vertex[];
  shape: RoofShape;
  overhang: number;
  base_offset: number;
  phase: Phase;
  name: string;
}

export interface Opening {
  host: string;
  kind: OpeningKind;
  offset: number;
  sill_override?: number;
  width?: number;
  height?: number;
  flip_hand: boolean;
  flip_facing: boolean;
  name: string;
  reveal_depth?: number;
  reveal_material?: string;
}

export interface Stair {
  storey: string;
  start: Point2;
  direction: number;
  width: number;
  flight: StairFlight;
  top: TopConstraint;
  max_riser: number;
  min_tread: number;
  stringer: StairStringer;
  nosing: number;
  tread_thickness: number;
  riser: RiserKind;
  landing_depth: number;
  phase: Phase;
  name: string;
}

export interface Railing {
  storey: string;
  path: Point2[];
  height: number;
  post_spacing: number;
  profile: Profile;
  post_profile: Profile;
  baluster?: Baluster;
  infill: Infill;
  material: string;
  base_offset: number;
  host?: RailingHost;
  phase: Phase;
  name: string;
}

export interface Ramp {
  storey: string;
  path: Vertex[];
  width: number;
  landing_start: number;
  landing_end: number;
  landing_turn: number;
  max_slope: number;
  thickness: number;
  material: string;
  base_offset: number;
  top: TopConstraint;
  railing_left: boolean;
  railing_right: boolean;
  name: string;
}

export interface Space {
  storey: string;
  number: string;
  name: string;
  boundary: SpaceBoundary;
  usage: string;
  phase: Phase;
  zone?: string;
  floor_finish?: string;
  wall_finish?: string;
  ceiling_finish?: string;
}

export interface Zone {
  name: string;
  category: string;
  occupancy_density: number;
}

export interface AreaScheme {
  name: string;
  measure: AreaMeasure;
  usages: string[];
  zones: string[];
}

export interface ViewPlane {
  start: Point2;
  end: Point2;
}

export interface ViewCrop {
  min: Point2;
  max: Point2;
}

export interface ViewCamera {
  target: Point2;
  target_height: number;
  azimuth: number;
  pitch: number;
  distance: number;
}

export interface View {
  building: string;
  name: string;
  kind: ViewKind;
  storey?: string;
  plane?: ViewPlane;
  camera?: ViewCamera;
  cut_height?: number;
  depth: number;
  crop?: ViewCrop;
  hidden: ViewCategory[];
  phase?: Phase;
  scale: number;
  detail: DetailLevel;
}

export interface ScheduleColumn {
  key: ScheduleKey;
  heading?: string;
  total: boolean;
}

export interface ScheduleSort {
  key: ScheduleKey;
  descending: boolean;
}

export interface ScheduleFilter {
  key: ScheduleKey;
  op: ScheduleOp;
  value: string;
}

export interface ScheduleGroup {
  key: ScheduleKey;
}

export interface Schedule {
  name: string;
  category: ScheduleCategory;
  columns: ScheduleColumn[];
  sort: ScheduleSort[];
  filter: ScheduleFilter[];
  group: ScheduleGroup[];
  itemize: boolean;
  storeys: string[];
  phases: Phase[];
}

export type PropertySet = Record<string, Record<string, PropertyValue>>;

export type ClassificationSet = Record<string, string>;

