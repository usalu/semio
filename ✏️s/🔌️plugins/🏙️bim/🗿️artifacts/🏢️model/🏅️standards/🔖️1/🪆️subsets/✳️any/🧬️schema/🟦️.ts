/** 🧬️ BIM model artifact schema — every field with its state class. */

export interface ModelArtifact {
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
  supports?: Record<string, StructuralSupport>;
  load_cases?: Record<string, LoadCase>;
  loads?: Record<string, StructuralLoad>;
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

export type MepSystem = "Supply" | "Return" | "Exhaust" | "DomesticWater" | "Waste" | "Gas" | "Power" | "Data" | "Lighting";

export type ElementClass = "Wall" | "CurtainWall" | "Window" | "Door" | "Column" | "Beam" | "Slab" | "Ceiling" | "Roof" | "Stair" | "Ramp" | "Railing" | "WallSweep" | "Component" | "Mep";

export type RuleKind = "MinClearHeight" | "MaxRiser" | "MinTread" | "MinStairWidth" | "MinDoorWidth" | "MaxRampSlope" | "MinCorridorWidth" | "MaxCompartmentArea";

export type RuleSeverity = "Error" | "Warning" | "Note";

export type IssueStatus = "Open" | "InProgress" | "Resolved" | "Closed";

export type IssuePriority = "Low" | "Normal" | "High" | "Critical";

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

export type ScheduleField = "Id" | "Name" | "Kind" | "Storey" | "Level" | "Type" | "Phase" | "Material" | "Host" | "Number" | "Usage" | "Surface" | "Swing" | "Leaves" | "Panes" | "Count" | "Length" | "Width" | "Height" | "Perimeter" | "GrossSideArea" | "OpeningArea" | "NetSideArea" | "GrossArea" | "NetArea" | "SurfaceArea" | "GrossVolume" | "NetVolume" | "Mass" | "Risers" | "Thickness" | "LayerArea" | "LayerVolume" | "LayerMass" | "FinishArea" | "UValue" | "GValue";

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

export type MepShape =
  | { Duct: { width: number; height: number } }
  | { Pipe: { diameter: number } }
  | { Tray: { width: number; height: number } };

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

export interface SpaceConditions {
  occupancy?: string;
  occupancy_density?: number;
  heating_setpoint?: number;
  cooling_setpoint?: number;
  ventilation_rate?: number;
  lighting_power_density?: number;
  equipment_power_density?: number;
  schedule?: string;
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

export interface Point3 {
  x: number;
  y: number;
  z: number;
}

export interface Component {
  storey: string;
  family: string;
  position: Point2;
  elevation: number;
  rotation: number;
  mirrored: boolean;
  host?: string;
  system?: MepSystem;
  name: string;
}

export interface ComponentOverride {
  component: string;
  name: string;
  value: string;
}

export interface MepElement {
  storey: string;
  system: MepSystem;
  shape: MepShape;
  path: Point3[];
  name: string;
}

export interface ElementSelector {
  classes: ElementClass[];
  storeys: string[];
  phases: Phase[];
  ids: string[];
}

export interface RuleScope {
  storeys: string[];
  phases: Phase[];
  ids: string[];
  filter: string;
}

export interface ClashRef {
  set: string;
  first: string;
  second: string;
}

export interface SectionBox {
  min: Point3;
  max: Point3;
}

export interface IssueViewpoint {
  camera: ViewCamera;
  section?: SectionBox;
  isolate: string[];
}

export interface ClashSet {
  name: string;
  a: ElementSelector;
  b: ElementSelector;
  tolerance: number;
  clearance: number;
}

export interface Rule {
  name: string;
  kind: RuleKind;
  limit: number;
  severity: RuleSeverity;
  scope: RuleScope;
}

export interface Issue {
  title: string;
  description: string;
  status: IssueStatus;
  priority: IssuePriority;
  assignee: string;
  author: string;
  created: string;
  labels: string[];
  elements: string[];
  clash?: ClashRef;
  viewpoint?: IssueViewpoint;
}

export interface IssueComment {
  issue: string;
  author: string;
  date: string;
  text: string;
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
  u_value?: number;
  g_value?: number;
  frame_fraction?: number;
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
  u_value?: number;
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
  u_value?: number;
  g_value?: number;
  frame_fraction?: number;
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


/** 🦴️ Authored SI structural support/load contracts. */
export type StructuralLocation = { Point: { station: number } } | { Line: { start: number; end: number } } | "Area";
export interface Restraints { x: boolean; y: boolean; z: boolean; rx: boolean; ry: boolean; rz: boolean; }
export interface StructuralSupport { name: string; member: string; location: StructuralLocation; offset: Point3; restraints: Restraints; }
export interface LoadCase { name: string; category: string; factor: number; }
export interface StructuralLoad { name: string; load_case: string; member: string; location: StructuralLocation; force: Point3; moment: Point3; }

/** 🧭️ Authored design-option and workset membership records. */
export interface OptionGroup {name:string;}
export interface DesignOption {group:string;name:string;primary:boolean;}
export interface Workset {name:string;default_visible:boolean;}
export interface ElementMembership {target:string;}

//#region 🚪️Parsers
/** 🛑️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class bimModelArtifactGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const bimModelArtifactGuardReject = (at: string, why: string): never => {
  throw new bimModelArtifactGuardRefusal(at, why);
};

type bimModelArtifactGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type bimModelArtifactGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number; readonly exclusiveMinimum?: number; readonly exclusiveMaximum?: number };
type bimModelArtifactGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const bimModelArtifactGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : bimModelArtifactGuardReject(at, "value is not an object");
export const bimModelArtifactGuardClosed = (row: Readonly<Record<string, unknown>>, at: string, keys: readonly string[]): Readonly<Record<string, unknown>> => {
  for (const key of Object.keys(row)) if (!keys.includes(key)) bimModelArtifactGuardReject(`${at}.${key}`, "unknown property");
  return row;
};
export const bimModelArtifactGuardExactObject = (value: unknown, at: string, fields: readonly string[], required: readonly string[] = fields): Readonly<Record<string, unknown>> => {
  const row = bimModelArtifactGuardClosed(bimModelArtifactGuardObject(value, at), at, fields);
  for (const field of required) if (!Object.hasOwn(row, field)) bimModelArtifactGuardReject(`${at}.${field}`, "field is missing");
  return row;
};
export const bimModelArtifactGuardArray = (value: unknown, at: string, bounds: bimModelArtifactGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return bimModelArtifactGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) bimModelArtifactGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) bimModelArtifactGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const bimModelArtifactGuardString = (value: unknown, at: string, bounds: bimModelArtifactGuardTextBounds = {}): string => {
  if (typeof value !== "string") return bimModelArtifactGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) bimModelArtifactGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) bimModelArtifactGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) bimModelArtifactGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const bimModelArtifactGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : bimModelArtifactGuardReject(at, "value is not a boolean"));
export const bimModelArtifactGuardNumber = (value: unknown, at: string, bounds: bimModelArtifactGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return bimModelArtifactGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) bimModelArtifactGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) bimModelArtifactGuardReject(at, `number is above ${bounds.maximum}`);
  if (bounds.exclusiveMinimum !== undefined && value <= bounds.exclusiveMinimum) bimModelArtifactGuardReject(at, `number is not above ${bounds.exclusiveMinimum}`);
  if (bounds.exclusiveMaximum !== undefined && value >= bounds.exclusiveMaximum) bimModelArtifactGuardReject(at, `number is not below ${bounds.exclusiveMaximum}`);
  return value;
};
export const bimModelArtifactGuardInteger = (value: unknown, at: string, bounds: bimModelArtifactGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? bimModelArtifactGuardNumber(value, at, bounds) : bimModelArtifactGuardReject(at, "value is not an integer");
export const bimModelArtifactGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : bimModelArtifactGuardReject(at, `value is not one of ${members.join(", ")}`);
export const bimModelArtifactGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : bimModelArtifactGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseAnchorEnd(value: unknown, at = "$"): AnchorEnd {
  return bimModelArtifactGuardMember(value, `${at}`, ["Start", "End"] as const);
}

export function parseAnnotationAnchor(value: unknown, at = "$"): AnnotationAnchor {
  return ((row0) => { if (row0["Point"] !== undefined) { bimModelArtifactGuardClosed(row0, `${at}`, ["Point"]); return { Point: ((row1) => ({ point: parsePoint2(row1["point"], `${at}.Point.point`) }))(bimModelArtifactGuardClosed(bimModelArtifactGuardObject(row0["Point"], `${at}.Point`), `${at}.Point`, ["point"])) }; } if (row0["WallFace"] !== undefined) { bimModelArtifactGuardClosed(row0, `${at}`, ["WallFace"]); return { WallFace: ((row1) => ({ wall: bimModelArtifactGuardString(row1["wall"], `${at}.WallFace.wall`), side: parseWallSide(row1["side"], `${at}.WallFace.side`) }))(bimModelArtifactGuardClosed(bimModelArtifactGuardObject(row0["WallFace"], `${at}.WallFace`), `${at}.WallFace`, ["wall", "side"])) }; } if (row0["WallAxis"] !== undefined) { bimModelArtifactGuardClosed(row0, `${at}`, ["WallAxis"]); return { WallAxis: ((row1) => ({ wall: bimModelArtifactGuardString(row1["wall"], `${at}.WallAxis.wall`) }))(bimModelArtifactGuardClosed(bimModelArtifactGuardObject(row0["WallAxis"], `${at}.WallAxis`), `${at}.WallAxis`, ["wall"])) }; } if (row0["WallEnd"] !== undefined) { bimModelArtifactGuardClosed(row0, `${at}`, ["WallEnd"]); return { WallEnd: ((row1) => ({ wall: bimModelArtifactGuardString(row1["wall"], `${at}.WallEnd.wall`), end: parseAnchorEnd(row1["end"], `${at}.WallEnd.end`) }))(bimModelArtifactGuardClosed(bimModelArtifactGuardObject(row0["WallEnd"], `${at}.WallEnd`), `${at}.WallEnd`, ["wall", "end"])) }; } if (row0["OpeningCentre"] !== undefined) { bimModelArtifactGuardClosed(row0, `${at}`, ["OpeningCentre"]); return { OpeningCentre: ((row1) => ({ opening: bimModelArtifactGuardString(row1["opening"], `${at}.OpeningCentre.opening`) }))(bimModelArtifactGuardClosed(bimModelArtifactGuardObject(row0["OpeningCentre"], `${at}.OpeningCentre`), `${at}.OpeningCentre`, ["opening"])) }; } if (row0["Grid"] !== undefined) { bimModelArtifactGuardClosed(row0, `${at}`, ["Grid"]); return { Grid: ((row1) => ({ grid: bimModelArtifactGuardString(row1["grid"], `${at}.Grid.grid`) }))(bimModelArtifactGuardClosed(bimModelArtifactGuardObject(row0["Grid"], `${at}.Grid`), `${at}.Grid`, ["grid"])) }; } if (row0["ColumnCentre"] !== undefined) { bimModelArtifactGuardClosed(row0, `${at}`, ["ColumnCentre"]); return { ColumnCentre: ((row1) => ({ column: bimModelArtifactGuardString(row1["column"], `${at}.ColumnCentre.column`) }))(bimModelArtifactGuardClosed(bimModelArtifactGuardObject(row0["ColumnCentre"], `${at}.ColumnCentre`), `${at}.ColumnCentre`, ["column"])) }; } return bimModelArtifactGuardReject(`${at}`, "value matches no variant"); })(bimModelArtifactGuardObject(value, `${at}`));
}

export function parseAnnotationStyle(value: unknown, at = "$"): AnnotationStyle {
  const row = bimModelArtifactGuardClosed(bimModelArtifactGuardObject(value, at), at, ["name", "text_height", "terminator", "unit", "precision", "mark_size", "gap", "overshoot"]);
  return {
    name: bimModelArtifactGuardString(row["name"], `${at}.name`),
    text_height: bimModelArtifactGuardNumber(row["text_height"], `${at}.text_height`),
    terminator: parseTerminator(row["terminator"], `${at}.terminator`),
    unit: parseDimensionUnit(row["unit"], `${at}.unit`),
    precision: bimModelArtifactGuardInteger(row["precision"], `${at}.precision`, {"minimum": 0, "maximum": 4294967295}),
    mark_size: bimModelArtifactGuardNumber(row["mark_size"], `${at}.mark_size`),
    gap: bimModelArtifactGuardNumber(row["gap"], `${at}.gap`),
    overshoot: bimModelArtifactGuardNumber(row["overshoot"], `${at}.overshoot`),
  };
}

export function parseAreaMeasure(value: unknown, at = "$"): AreaMeasure {
  return bimModelArtifactGuardMember(value, `${at}`, ["Gross", "Net"] as const);
}

export function parseAreaScheme(value: unknown, at = "$"): AreaScheme {
  const row = bimModelArtifactGuardClosed(bimModelArtifactGuardObject(value, at), at, ["name", "measure", "usages", "zones"]);
  return {
    name: bimModelArtifactGuardString(row["name"], `${at}.name`),
    measure: parseAreaMeasure(row["measure"], `${at}.measure`),
    usages: bimModelArtifactGuardArray(row["usages"], `${at}.usages`).map((item, index) => bimModelArtifactGuardString(item, `${at}.usages[${index}]`)),
    zones: bimModelArtifactGuardArray(row["zones"], `${at}.zones`).map((item, index) => bimModelArtifactGuardString(item, `${at}.zones[${index}]`)),
  };
}

export function parseAxis(value: unknown, at = "$"): Axis {
  return ((row0) => { if (row0["Line"] !== undefined) { bimModelArtifactGuardClosed(row0, `${at}`, ["Line"]); return { Line: ((row1) => ({ start: parsePoint2(row1["start"], `${at}.Line.start`), end: parsePoint2(row1["end"], `${at}.Line.end`) }))(bimModelArtifactGuardClosed(bimModelArtifactGuardObject(row0["Line"], `${at}.Line`), `${at}.Line`, ["start", "end"])) }; } if (row0["Arc"] !== undefined) { bimModelArtifactGuardClosed(row0, `${at}`, ["Arc"]); return { Arc: ((row1) => ({ start: parsePoint2(row1["start"], `${at}.Arc.start`), end: parsePoint2(row1["end"], `${at}.Arc.end`), bulge: bimModelArtifactGuardNumber(row1["bulge"], `${at}.Arc.bulge`) }))(bimModelArtifactGuardClosed(bimModelArtifactGuardObject(row0["Arc"], `${at}.Arc`), `${at}.Arc`, ["start", "end", "bulge"])) }; } return bimModelArtifactGuardReject(`${at}`, "value matches no variant"); })(bimModelArtifactGuardObject(value, `${at}`));
}

export function parseBaluster(value: unknown, at = "$"): Baluster {
  const row = bimModelArtifactGuardClosed(bimModelArtifactGuardObject(value, at), at, ["profile", "spacing"]);
  return {
    profile: parseProfile(row["profile"], `${at}.profile`),
    spacing: bimModelArtifactGuardNumber(row["spacing"], `${at}.spacing`),
  };
}

export function parseBeam(value: unknown, at = "$"): Beam {
  const row = bimModelArtifactGuardClosed(bimModelArtifactGuardObject(value, at), at, ["storey", "beam_type", "axis", "top_offset", "end_top_offset", "phase", "name"]);
  return {
    storey: bimModelArtifactGuardString(row["storey"], `${at}.storey`),
    beam_type: bimModelArtifactGuardString(row["beam_type"], `${at}.beam_type`),
    axis: parseAxis(row["axis"], `${at}.axis`),
    top_offset: bimModelArtifactGuardNumber(row["top_offset"], `${at}.top_offset`),
    end_top_offset: row["end_top_offset"] === undefined ? undefined : bimModelArtifactGuardNumber(row["end_top_offset"], `${at}.end_top_offset`),
    phase: parsePhase(row["phase"], `${at}.phase`),
    name: bimModelArtifactGuardString(row["name"], `${at}.name`),
  };
}

export function parseBeamType(value: unknown, at = "$"): BeamType {
  const row = bimModelArtifactGuardClosed(bimModelArtifactGuardObject(value, at), at, ["name", "profile", "material"]);
  return {
    name: bimModelArtifactGuardString(row["name"], `${at}.name`),
    profile: parseProfile(row["profile"], `${at}.profile`),
    material: bimModelArtifactGuardString(row["material"], `${at}.material`),
  };
}

export function parseBuilding(value: unknown, at = "$"): Building {
  const row = bimModelArtifactGuardClosed(bimModelArtifactGuardObject(value, at), at, ["site", "name", "origin", "rotation", "elevation"]);
  return {
    site: bimModelArtifactGuardString(row["site"], `${at}.site`),
    name: bimModelArtifactGuardString(row["name"], `${at}.name`),
    origin: parsePoint2(row["origin"], `${at}.origin`),
    rotation: bimModelArtifactGuardNumber(row["rotation"], `${at}.rotation`),
    elevation: bimModelArtifactGuardNumber(row["elevation"], `${at}.elevation`),
  };
}

export function parseCeiling(value: unknown, at = "$"): Ceiling {
  const row = bimModelArtifactGuardClosed(bimModelArtifactGuardObject(value, at), at, ["storey", "ceiling_type", "boundary", "holes", "offset", "slope", "name"]);
  return {
    storey: bimModelArtifactGuardString(row["storey"], `${at}.storey`),
    ceiling_type: bimModelArtifactGuardString(row["ceiling_type"], `${at}.ceiling_type`),
    boundary: bimModelArtifactGuardArray(row["boundary"], `${at}.boundary`).map((item, index) => parseVertex(item, `${at}.boundary[${index}]`)),
    holes: bimModelArtifactGuardArray(row["holes"], `${at}.holes`).map((item, index) => bimModelArtifactGuardArray(item, `${at}.holes[${index}]`).map((item1, index1) => parseVertex(item1, `${at}.holes[${index}][${index1}]`))),
    offset: bimModelArtifactGuardNumber(row["offset"], `${at}.offset`),
    slope: row["slope"] === undefined ? undefined : parseSlope(row["slope"], `${at}.slope`),
    name: bimModelArtifactGuardString(row["name"], `${at}.name`),
  };
}

export function parseCeilingType(value: unknown, at = "$"): CeilingType {
  const row = bimModelArtifactGuardClosed(bimModelArtifactGuardObject(value, at), at, ["name", "layers"]);
  return {
    name: bimModelArtifactGuardString(row["name"], `${at}.name`),
    layers: bimModelArtifactGuardArray(row["layers"], `${at}.layers`).map((item, index) => parseLayer(item, `${at}.layers[${index}]`)),
  };
}

export function parseClashRef(value: unknown, at = "$"): ClashRef {
  const row = bimModelArtifactGuardClosed(bimModelArtifactGuardObject(value, at), at, ["set", "first", "second"]);
  return {
    set: bimModelArtifactGuardString(row["set"], `${at}.set`),
    first: bimModelArtifactGuardString(row["first"], `${at}.first`),
    second: bimModelArtifactGuardString(row["second"], `${at}.second`),
  };
}

export function parseClashSet(value: unknown, at = "$"): ClashSet {
  const row = bimModelArtifactGuardClosed(bimModelArtifactGuardObject(value, at), at, ["name", "a", "b", "tolerance", "clearance"]);
  return {
    name: bimModelArtifactGuardString(row["name"], `${at}.name`),
    a: parseElementSelector(row["a"], `${at}.a`),
    b: parseElementSelector(row["b"], `${at}.b`),
    tolerance: bimModelArtifactGuardNumber(row["tolerance"], `${at}.tolerance`),
    clearance: bimModelArtifactGuardNumber(row["clearance"], `${at}.clearance`),
  };
}

export function parseClassificationItem(value: unknown, at = "$"): ClassificationItem {
  const row = bimModelArtifactGuardClosed(bimModelArtifactGuardObject(value, at), at, ["code", "title", "parent"]);
  return {
    code: bimModelArtifactGuardString(row["code"], `${at}.code`),
    title: bimModelArtifactGuardString(row["title"], `${at}.title`),
    parent: row["parent"] === undefined ? undefined : bimModelArtifactGuardString(row["parent"], `${at}.parent`),
  };
}

export function parseClassificationSet(value: unknown, at = "$"): ClassificationSet {
  return Object.fromEntries(Object.entries(bimModelArtifactGuardObject(value, `${at}`)).map(([key0, value0]) => [key0, bimModelArtifactGuardString(value0, `${at}.${key0}`)]));
}

export function parseClassificationSystem(value: unknown, at = "$"): ClassificationSystem {
  const row = bimModelArtifactGuardClosed(bimModelArtifactGuardObject(value, at), at, ["name", "edition", "source", "entries"]);
  return {
    name: bimModelArtifactGuardString(row["name"], `${at}.name`),
    edition: bimModelArtifactGuardString(row["edition"], `${at}.edition`),
    source: row["source"] === undefined ? undefined : bimModelArtifactGuardString(row["source"], `${at}.source`),
    entries: bimModelArtifactGuardArray(row["entries"], `${at}.entries`).map((item, index) => parseClassificationItem(item, `${at}.entries[${index}]`)),
  };
}

export function parseColumn(value: unknown, at = "$"): Column {
  const row = bimModelArtifactGuardClosed(bimModelArtifactGuardObject(value, at), at, ["storey", "column_type", "position", "rotation", "tilt", "base_offset", "top", "phase", "name"]);
  return {
    storey: bimModelArtifactGuardString(row["storey"], `${at}.storey`),
    column_type: bimModelArtifactGuardString(row["column_type"], `${at}.column_type`),
    position: parsePoint2(row["position"], `${at}.position`),
    rotation: bimModelArtifactGuardNumber(row["rotation"], `${at}.rotation`),
    tilt: row["tilt"] === undefined ? undefined : parseSlope(row["tilt"], `${at}.tilt`),
    base_offset: bimModelArtifactGuardNumber(row["base_offset"], `${at}.base_offset`),
    top: parseTopConstraint(row["top"], `${at}.top`),
    phase: parsePhase(row["phase"], `${at}.phase`),
    name: bimModelArtifactGuardString(row["name"], `${at}.name`),
  };
}

export function parseColumnType(value: unknown, at = "$"): ColumnType {
  const row = bimModelArtifactGuardClosed(bimModelArtifactGuardObject(value, at), at, ["name", "profile", "material"]);
  return {
    name: bimModelArtifactGuardString(row["name"], `${at}.name`),
    profile: parseProfile(row["profile"], `${at}.profile`),
    material: bimModelArtifactGuardString(row["material"], `${at}.material`),
  };
}

export function parseComponent(value: unknown, at = "$"): Component {
  const row = bimModelArtifactGuardClosed(bimModelArtifactGuardObject(value, at), at, ["storey", "family", "position", "elevation", "rotation", "mirrored", "host", "system", "name"]);
  return {
    storey: bimModelArtifactGuardString(row["storey"], `${at}.storey`),
    family: bimModelArtifactGuardString(row["family"], `${at}.family`),
    position: parsePoint2(row["position"], `${at}.position`),
    elevation: bimModelArtifactGuardNumber(row["elevation"], `${at}.elevation`),
    rotation: bimModelArtifactGuardNumber(row["rotation"], `${at}.rotation`),
    mirrored: bimModelArtifactGuardBoolean(row["mirrored"], `${at}.mirrored`),
    host: row["host"] === undefined ? undefined : bimModelArtifactGuardString(row["host"], `${at}.host`),
    system: row["system"] === undefined ? undefined : parseMepSystem(row["system"], `${at}.system`),
    name: bimModelArtifactGuardString(row["name"], `${at}.name`),
  };
}

export function parseComponentOverride(value: unknown, at = "$"): ComponentOverride {
  const row = bimModelArtifactGuardClosed(bimModelArtifactGuardObject(value, at), at, ["component", "name", "value"]);
  return {
    component: bimModelArtifactGuardString(row["component"], `${at}.component`),
    name: bimModelArtifactGuardString(row["name"], `${at}.name`),
    value: bimModelArtifactGuardString(row["value"], `${at}.value`),
  };
}

export function parseCurtainGrid(value: unknown, at = "$"): CurtainGrid {
  return ((row0) => { if (row0["Spacing"] !== undefined) { bimModelArtifactGuardClosed(row0, `${at}`, ["Spacing"]); return { Spacing: ((row1) => ({ spacing: bimModelArtifactGuardNumber(row1["spacing"], `${at}.Spacing.spacing`) }))(bimModelArtifactGuardClosed(bimModelArtifactGuardObject(row0["Spacing"], `${at}.Spacing`), `${at}.Spacing`, ["spacing"])) }; } if (row0["Lines"] !== undefined) { bimModelArtifactGuardClosed(row0, `${at}`, ["Lines"]); return { Lines: ((row1) => ({ positions: bimModelArtifactGuardArray(row1["positions"], `${at}.Lines.positions`).map((item2, index2) => bimModelArtifactGuardNumber(item2, `${at}.Lines.positions[${index2}]`)) }))(bimModelArtifactGuardClosed(bimModelArtifactGuardObject(row0["Lines"], `${at}.Lines`), `${at}.Lines`, ["positions"])) }; } return bimModelArtifactGuardReject(`${at}`, "value matches no variant"); })(bimModelArtifactGuardObject(value, `${at}`));
}

export function parseCurtainPanel(value: unknown, at = "$"): CurtainPanel {
  return ((input0: unknown) => { if (typeof input0 === "string") { try { return bimModelArtifactGuardConstant(input0, `${at}`, "Glass"); } catch (error) { if (!(error instanceof bimModelArtifactGuardRefusal)) throw error; } } if (typeof input0 === "object" && input0 !== null && !Array.isArray(input0)) { try { return ((row1) => ({ Solid: ((row2) => ({ material: bimModelArtifactGuardString(row2["material"], `${at}.Solid.material`) }))(bimModelArtifactGuardClosed(bimModelArtifactGuardObject(row1["Solid"], `${at}.Solid`), `${at}.Solid`, ["material"])) }))(bimModelArtifactGuardClosed(bimModelArtifactGuardObject(input0, `${at}`), `${at}`, ["Solid"])); } catch (error) { if (!(error instanceof bimModelArtifactGuardRefusal)) throw error; } } if (typeof input0 === "object" && input0 !== null && !Array.isArray(input0)) { try { return ((row1) => ({ Door: ((row2) => ({ door_type: bimModelArtifactGuardString(row2["door_type"], `${at}.Door.door_type`) }))(bimModelArtifactGuardClosed(bimModelArtifactGuardObject(row1["Door"], `${at}.Door`), `${at}.Door`, ["door_type"])) }))(bimModelArtifactGuardClosed(bimModelArtifactGuardObject(input0, `${at}`), `${at}`, ["Door"])); } catch (error) { if (!(error instanceof bimModelArtifactGuardRefusal)) throw error; } } if (typeof input0 === "object" && input0 !== null && !Array.isArray(input0)) { try { return ((row1) => ({ Window: ((row2) => ({ window_type: bimModelArtifactGuardString(row2["window_type"], `${at}.Window.window_type`) }))(bimModelArtifactGuardClosed(bimModelArtifactGuardObject(row1["Window"], `${at}.Window`), `${at}.Window`, ["window_type"])) }))(bimModelArtifactGuardClosed(bimModelArtifactGuardObject(input0, `${at}`), `${at}`, ["Window"])); } catch (error) { if (!(error instanceof bimModelArtifactGuardRefusal)) throw error; } } if (typeof input0 === "string") { try { return bimModelArtifactGuardConstant(input0, `${at}`, "Empty"); } catch (error) { if (!(error instanceof bimModelArtifactGuardRefusal)) throw error; } } return bimModelArtifactGuardReject(`${at}`, "value matches no variant"); })(value);
}

export function parseCurtainPanelOverride(value: unknown, at = "$"): CurtainPanelOverride {
  const row = bimModelArtifactGuardClosed(bimModelArtifactGuardObject(value, at), at, ["curtain", "u", "v", "panel"]);
  return {
    curtain: bimModelArtifactGuardString(row["curtain"], `${at}.curtain`),
    u: bimModelArtifactGuardInteger(row["u"], `${at}.u`, {"minimum": 0, "maximum": 4294967295}),
    v: bimModelArtifactGuardInteger(row["v"], `${at}.v`, {"minimum": 0, "maximum": 4294967295}),
    panel: parseCurtainPanel(row["panel"], `${at}.panel`),
  };
}

export function parseCurtainWall(value: unknown, at = "$"): CurtainWall {
  const row = bimModelArtifactGuardClosed(bimModelArtifactGuardObject(value, at), at, ["storey", "curtain_wall_type", "axis", "base_offset", "top", "u_grid", "v_grid", "phase", "name"]);
  return {
    storey: bimModelArtifactGuardString(row["storey"], `${at}.storey`),
    curtain_wall_type: bimModelArtifactGuardString(row["curtain_wall_type"], `${at}.curtain_wall_type`),
    axis: parseAxis(row["axis"], `${at}.axis`),
    base_offset: bimModelArtifactGuardNumber(row["base_offset"], `${at}.base_offset`),
    top: parseTopConstraint(row["top"], `${at}.top`),
    u_grid: row["u_grid"] === undefined ? undefined : parseCurtainGrid(row["u_grid"], `${at}.u_grid`),
    v_grid: row["v_grid"] === undefined ? undefined : parseCurtainGrid(row["v_grid"], `${at}.v_grid`),
    phase: parsePhase(row["phase"], `${at}.phase`),
    name: bimModelArtifactGuardString(row["name"], `${at}.name`),
  };
}

export function parseCurtainWallType(value: unknown, at = "$"): CurtainWallType {
  const row = bimModelArtifactGuardClosed(bimModelArtifactGuardObject(value, at), at, ["name", "u_grid", "v_grid", "interior_mullion", "border_mullion", "panel", "panel_material", "mullion_material", "u_value", "g_value", "frame_fraction"]);
  return {
    name: bimModelArtifactGuardString(row["name"], `${at}.name`),
    u_grid: parseCurtainGrid(row["u_grid"], `${at}.u_grid`),
    v_grid: parseCurtainGrid(row["v_grid"], `${at}.v_grid`),
    interior_mullion: parseProfile(row["interior_mullion"], `${at}.interior_mullion`),
    border_mullion: parseProfile(row["border_mullion"], `${at}.border_mullion`),
    panel: parseCurtainPanel(row["panel"], `${at}.panel`),
    panel_material: bimModelArtifactGuardString(row["panel_material"], `${at}.panel_material`),
    mullion_material: bimModelArtifactGuardString(row["mullion_material"], `${at}.mullion_material`),
    u_value: row["u_value"] === undefined ? undefined : bimModelArtifactGuardNumber(row["u_value"], `${at}.u_value`),
    g_value: row["g_value"] === undefined ? undefined : bimModelArtifactGuardNumber(row["g_value"], `${at}.g_value`),
    frame_fraction: row["frame_fraction"] === undefined ? undefined : bimModelArtifactGuardNumber(row["frame_fraction"], `${at}.frame_fraction`),
  };
}

export function parseDesignOption(value: unknown, at = "$"): DesignOption {
  const row = bimModelArtifactGuardClosed(bimModelArtifactGuardObject(value, at), at, ["group", "name", "primary"]);
  return {
    group: bimModelArtifactGuardString(row["group"], `${at}.group`),
    name: bimModelArtifactGuardString(row["name"], `${at}.name`, {"minLength": 1}),
    primary: bimModelArtifactGuardBoolean(row["primary"], `${at}.primary`),
  };
}

export function parseDetailLevel(value: unknown, at = "$"): DetailLevel {
  return bimModelArtifactGuardMember(value, `${at}`, ["Coarse", "Medium", "Fine"] as const);
}

export function parseDimension(value: unknown, at = "$"): Dimension {
  const row = bimModelArtifactGuardClosed(bimModelArtifactGuardObject(value, at), at, ["storey", "anchors", "angle", "offset", "style", "lock", "name"]);
  return {
    storey: bimModelArtifactGuardString(row["storey"], `${at}.storey`),
    anchors: bimModelArtifactGuardArray(row["anchors"], `${at}.anchors`).map((item, index) => parseAnnotationAnchor(item, `${at}.anchors[${index}]`)),
    angle: bimModelArtifactGuardNumber(row["angle"], `${at}.angle`),
    offset: bimModelArtifactGuardNumber(row["offset"], `${at}.offset`),
    style: bimModelArtifactGuardString(row["style"], `${at}.style`),
    lock: row["lock"] === undefined ? undefined : bimModelArtifactGuardNumber(row["lock"], `${at}.lock`),
    name: bimModelArtifactGuardString(row["name"], `${at}.name`),
  };
}

export function parseDimensionUnit(value: unknown, at = "$"): DimensionUnit {
  return bimModelArtifactGuardMember(value, `${at}`, ["Metre", "Centimetre", "Millimetre"] as const);
}

export function parseDoorLeaves(value: unknown, at = "$"): DoorLeaves {
  return bimModelArtifactGuardMember(value, `${at}`, ["Single", "Double"] as const);
}

export function parseDoorType(value: unknown, at = "$"): DoorType {
  const row = bimModelArtifactGuardClosed(bimModelArtifactGuardObject(value, at), at, ["name", "width", "height", "frame_width", "frame_depth", "leaves", "swing", "material", "u_value"]);
  return {
    name: bimModelArtifactGuardString(row["name"], `${at}.name`),
    width: bimModelArtifactGuardNumber(row["width"], `${at}.width`),
    height: bimModelArtifactGuardNumber(row["height"], `${at}.height`),
    frame_width: bimModelArtifactGuardNumber(row["frame_width"], `${at}.frame_width`),
    frame_depth: bimModelArtifactGuardNumber(row["frame_depth"], `${at}.frame_depth`),
    leaves: parseDoorLeaves(row["leaves"], `${at}.leaves`),
    swing: parseSwing(row["swing"], `${at}.swing`),
    material: bimModelArtifactGuardString(row["material"], `${at}.material`),
    u_value: row["u_value"] === undefined ? undefined : bimModelArtifactGuardNumber(row["u_value"], `${at}.u_value`),
  };
}

export function parseElementClass(value: unknown, at = "$"): ElementClass {
  return bimModelArtifactGuardMember(value, `${at}`, ["Wall", "CurtainWall", "Window", "Door", "Column", "Beam", "Slab", "Ceiling", "Roof", "Stair", "Ramp", "Railing", "WallSweep", "Component", "Mep"] as const);
}

export function parseElementMembership(value: unknown, at = "$"): ElementMembership {
  const row = bimModelArtifactGuardClosed(bimModelArtifactGuardObject(value, at), at, ["target"]);
  return {
    target: bimModelArtifactGuardString(row["target"], `${at}.target`),
  };
}

export function parseElementSelector(value: unknown, at = "$"): ElementSelector {
  const row = bimModelArtifactGuardClosed(bimModelArtifactGuardObject(value, at), at, ["classes", "storeys", "phases", "ids"]);
  return {
    classes: bimModelArtifactGuardArray(row["classes"], `${at}.classes`).map((item, index) => parseElementClass(item, `${at}.classes[${index}]`)),
    storeys: bimModelArtifactGuardArray(row["storeys"], `${at}.storeys`).map((item, index) => bimModelArtifactGuardString(item, `${at}.storeys[${index}]`)),
    phases: bimModelArtifactGuardArray(row["phases"], `${at}.phases`).map((item, index) => parsePhase(item, `${at}.phases[${index}]`)),
    ids: bimModelArtifactGuardArray(row["ids"], `${at}.ids`).map((item, index) => bimModelArtifactGuardString(item, `${at}.ids[${index}]`)),
  };
}

export function parseEndJoin(value: unknown, at = "$"): EndJoin {
  return bimModelArtifactGuardMember(value, `${at}`, ["Miter", "Butt", "None"] as const);
}

export function parseExprPoint(value: unknown, at = "$"): ExprPoint {
  const row = bimModelArtifactGuardClosed(bimModelArtifactGuardObject(value, at), at, ["x", "y"]);
  return {
    x: bimModelArtifactGuardString(row["x"], `${at}.x`),
    y: bimModelArtifactGuardString(row["y"], `${at}.y`),
  };
}

export function parseExprPoint3(value: unknown, at = "$"): ExprPoint3 {
  const row = bimModelArtifactGuardClosed(bimModelArtifactGuardObject(value, at), at, ["x", "y", "z"]);
  return {
    x: bimModelArtifactGuardString(row["x"], `${at}.x`),
    y: bimModelArtifactGuardString(row["y"], `${at}.y`),
    z: bimModelArtifactGuardString(row["z"], `${at}.z`),
  };
}

export function parseFamily(value: unknown, at = "$"): Family {
  const row = bimModelArtifactGuardClosed(bimModelArtifactGuardObject(value, at), at, ["name", "category"]);
  return {
    name: bimModelArtifactGuardString(row["name"], `${at}.name`),
    category: parseFamilyCategory(row["category"], `${at}.category`),
  };
}

export function parseFamilyCategory(value: unknown, at = "$"): FamilyCategory {
  return bimModelArtifactGuardMember(value, `${at}`, ["Furniture", "Equipment", "Casework", "Plumbing", "Lighting", "Mechanical", "Electrical", "Generic", "Profile"] as const);
}

export function parseFamilyParameter(value: unknown, at = "$"): FamilyParameter {
  const row = bimModelArtifactGuardClosed(bimModelArtifactGuardObject(value, at), at, ["family", "name", "kind", "value"]);
  return {
    family: bimModelArtifactGuardString(row["family"], `${at}.family`),
    name: bimModelArtifactGuardString(row["name"], `${at}.name`),
    kind: parseParameterKind(row["kind"], `${at}.kind`),
    value: bimModelArtifactGuardString(row["value"], `${at}.value`),
  };
}

export function parseFamilySolid(value: unknown, at = "$"): FamilySolid {
  const row = bimModelArtifactGuardClosed(bimModelArtifactGuardObject(value, at), at, ["family", "name", "shape", "material", "visible", "offset"]);
  return {
    family: bimModelArtifactGuardString(row["family"], `${at}.family`),
    name: bimModelArtifactGuardString(row["name"], `${at}.name`),
    shape: parseSolidShape(row["shape"], `${at}.shape`),
    material: bimModelArtifactGuardString(row["material"], `${at}.material`),
    visible: bimModelArtifactGuardString(row["visible"], `${at}.visible`),
    offset: parseExprPoint3(row["offset"], `${at}.offset`),
  };
}

export function parseGridLine(value: unknown, at = "$"): GridLine {
  const row = bimModelArtifactGuardClosed(bimModelArtifactGuardObject(value, at), at, ["building", "label", "start", "end"]);
  return {
    building: bimModelArtifactGuardString(row["building"], `${at}.building`),
    label: bimModelArtifactGuardString(row["label"], `${at}.label`),
    start: parsePoint2(row["start"], `${at}.start`),
    end: parsePoint2(row["end"], `${at}.end`),
  };
}

export function parseHostSide(value: unknown, at = "$"): HostSide {
  return bimModelArtifactGuardMember(value, `${at}`, ["Left", "Right"] as const);
}

export function parseInfill(value: unknown, at = "$"): Infill {
  return ((input0: unknown) => { if (typeof input0 === "string") { try { return bimModelArtifactGuardConstant(input0, `${at}`, "None"); } catch (error) { if (!(error instanceof bimModelArtifactGuardRefusal)) throw error; } } if (typeof input0 === "object" && input0 !== null && !Array.isArray(input0)) { try { return ((row1) => ({ Glass: ((row2) => ({ thickness: bimModelArtifactGuardNumber(row2["thickness"], `${at}.Glass.thickness`) }))(bimModelArtifactGuardClosed(bimModelArtifactGuardObject(row1["Glass"], `${at}.Glass`), `${at}.Glass`, ["thickness"])) }))(bimModelArtifactGuardClosed(bimModelArtifactGuardObject(input0, `${at}`), `${at}`, ["Glass"])); } catch (error) { if (!(error instanceof bimModelArtifactGuardRefusal)) throw error; } } if (typeof input0 === "object" && input0 !== null && !Array.isArray(input0)) { try { return ((row1) => ({ Panel: ((row2) => ({ thickness: bimModelArtifactGuardNumber(row2["thickness"], `${at}.Panel.thickness`) }))(bimModelArtifactGuardClosed(bimModelArtifactGuardObject(row1["Panel"], `${at}.Panel`), `${at}.Panel`, ["thickness"])) }))(bimModelArtifactGuardClosed(bimModelArtifactGuardObject(input0, `${at}`), `${at}`, ["Panel"])); } catch (error) { if (!(error instanceof bimModelArtifactGuardRefusal)) throw error; } } return bimModelArtifactGuardReject(`${at}`, "value matches no variant"); })(value);
}

export function parseIsoSize(value: unknown, at = "$"): IsoSize {
  return bimModelArtifactGuardMember(value, `${at}`, ["A0", "A1", "A2", "A3", "A4"] as const);
}

export function parseIssue(value: unknown, at = "$"): Issue {
  const row = bimModelArtifactGuardClosed(bimModelArtifactGuardObject(value, at), at, ["title", "description", "status", "priority", "assignee", "author", "created", "labels", "elements", "clash", "viewpoint"]);
  return {
    title: bimModelArtifactGuardString(row["title"], `${at}.title`),
    description: bimModelArtifactGuardString(row["description"], `${at}.description`),
    status: parseIssueStatus(row["status"], `${at}.status`),
    priority: parseIssuePriority(row["priority"], `${at}.priority`),
    assignee: bimModelArtifactGuardString(row["assignee"], `${at}.assignee`),
    author: bimModelArtifactGuardString(row["author"], `${at}.author`),
    created: bimModelArtifactGuardString(row["created"], `${at}.created`),
    labels: bimModelArtifactGuardArray(row["labels"], `${at}.labels`).map((item, index) => bimModelArtifactGuardString(item, `${at}.labels[${index}]`)),
    elements: bimModelArtifactGuardArray(row["elements"], `${at}.elements`).map((item, index) => bimModelArtifactGuardString(item, `${at}.elements[${index}]`)),
    clash: row["clash"] === undefined ? undefined : parseClashRef(row["clash"], `${at}.clash`),
    viewpoint: row["viewpoint"] === undefined ? undefined : parseIssueViewpoint(row["viewpoint"], `${at}.viewpoint`),
  };
}

export function parseIssueComment(value: unknown, at = "$"): IssueComment {
  const row = bimModelArtifactGuardClosed(bimModelArtifactGuardObject(value, at), at, ["issue", "author", "date", "text"]);
  return {
    issue: bimModelArtifactGuardString(row["issue"], `${at}.issue`),
    author: bimModelArtifactGuardString(row["author"], `${at}.author`),
    date: bimModelArtifactGuardString(row["date"], `${at}.date`),
    text: bimModelArtifactGuardString(row["text"], `${at}.text`),
  };
}

export function parseIssuePriority(value: unknown, at = "$"): IssuePriority {
  return bimModelArtifactGuardMember(value, `${at}`, ["Low", "Normal", "High", "Critical"] as const);
}

export function parseIssueStatus(value: unknown, at = "$"): IssueStatus {
  return bimModelArtifactGuardMember(value, `${at}`, ["Open", "InProgress", "Resolved", "Closed"] as const);
}

export function parseIssueViewpoint(value: unknown, at = "$"): IssueViewpoint {
  const row = bimModelArtifactGuardClosed(bimModelArtifactGuardObject(value, at), at, ["camera", "section", "isolate"]);
  return {
    camera: parseViewCamera(row["camera"], `${at}.camera`),
    section: row["section"] === undefined ? undefined : parseSectionBox(row["section"], `${at}.section`),
    isolate: bimModelArtifactGuardArray(row["isolate"], `${at}.isolate`).map((item, index) => bimModelArtifactGuardString(item, `${at}.isolate[${index}]`)),
  };
}

export function parseLayer(value: unknown, at = "$"): Layer {
  const row = bimModelArtifactGuardClosed(bimModelArtifactGuardObject(value, at), at, ["material", "thickness", "function"]);
  return {
    material: bimModelArtifactGuardString(row["material"], `${at}.material`),
    thickness: bimModelArtifactGuardNumber(row["thickness"], `${at}.thickness`),
    function: parseLayerFunction(row["function"], `${at}.function`),
  };
}

export function parseLayerFunction(value: unknown, at = "$"): LayerFunction {
  return bimModelArtifactGuardMember(value, `${at}`, ["Structure", "Substrate", "Insulation", "Finish", "Membrane", "Core"] as const);
}

export function parseLeader(value: unknown, at = "$"): Leader {
  const row = bimModelArtifactGuardClosed(bimModelArtifactGuardObject(value, at), at, ["storey", "anchor", "offset", "text", "style"]);
  return {
    storey: bimModelArtifactGuardString(row["storey"], `${at}.storey`),
    anchor: parseAnnotationAnchor(row["anchor"], `${at}.anchor`),
    offset: parsePoint2(row["offset"], `${at}.offset`),
    text: bimModelArtifactGuardString(row["text"], `${at}.text`),
    style: bimModelArtifactGuardString(row["style"], `${at}.style`),
  };
}

export interface LineLocation {
  readonly start: number;
  readonly end: number;
}

export function parseLineLocation(value: unknown, at = "$"): LineLocation {
  const row = bimModelArtifactGuardClosed(bimModelArtifactGuardObject(value, at), at, ["start", "end"]);
  return {
    start: bimModelArtifactGuardNumber(row["start"], `${at}.start`, {"minimum": 0, "maximum": 1}),
    end: bimModelArtifactGuardNumber(row["end"], `${at}.end`, {"minimum": 0, "maximum": 1}),
  };
}

export function parseLoadCase(value: unknown, at = "$"): LoadCase {
  const row = bimModelArtifactGuardClosed(bimModelArtifactGuardObject(value, at), at, ["name", "category", "factor"]);
  return {
    name: bimModelArtifactGuardString(row["name"], `${at}.name`),
    category: bimModelArtifactGuardString(row["category"], `${at}.category`),
    factor: bimModelArtifactGuardNumber(row["factor"], `${at}.factor`),
  };
}

export function parseLocationLine(value: unknown, at = "$"): LocationLine {
  return bimModelArtifactGuardMember(value, `${at}`, ["Center", "Interior", "Exterior", "CoreCenter"] as const);
}

export function parseMaterial(value: unknown, at = "$"): Material {
  const row = bimModelArtifactGuardClosed(bimModelArtifactGuardObject(value, at), at, ["name", "category", "color", "density", "conductivity", "specific_heat"]);
  return {
    name: bimModelArtifactGuardString(row["name"], `${at}.name`),
    category: parseMaterialCategory(row["category"], `${at}.category`),
    color: parseRgb(row["color"], `${at}.color`),
    density: bimModelArtifactGuardNumber(row["density"], `${at}.density`),
    conductivity: bimModelArtifactGuardNumber(row["conductivity"], `${at}.conductivity`),
    specific_heat: bimModelArtifactGuardNumber(row["specific_heat"], `${at}.specific_heat`),
  };
}

export function parseMaterialCategory(value: unknown, at = "$"): MaterialCategory {
  return bimModelArtifactGuardMember(value, `${at}`, ["Concrete", "Masonry", "Wood", "Metal", "Glass", "Insulation", "Finish", "Membrane", "Other"] as const);
}

export function parseMepElement(value: unknown, at = "$"): MepElement {
  const row = bimModelArtifactGuardClosed(bimModelArtifactGuardObject(value, at), at, ["storey", "system", "shape", "path", "name"]);
  return {
    storey: bimModelArtifactGuardString(row["storey"], `${at}.storey`),
    system: parseMepSystem(row["system"], `${at}.system`),
    shape: parseMepShape(row["shape"], `${at}.shape`),
    path: bimModelArtifactGuardArray(row["path"], `${at}.path`).map((item, index) => parsePoint3(item, `${at}.path[${index}]`)),
    name: bimModelArtifactGuardString(row["name"], `${at}.name`),
  };
}

export function parseMepShape(value: unknown, at = "$"): MepShape {
  return ((row0) => { if (row0["Duct"] !== undefined) { bimModelArtifactGuardClosed(row0, `${at}`, ["Duct"]); return { Duct: ((row1) => ({ width: bimModelArtifactGuardNumber(row1["width"], `${at}.Duct.width`), height: bimModelArtifactGuardNumber(row1["height"], `${at}.Duct.height`) }))(bimModelArtifactGuardClosed(bimModelArtifactGuardObject(row0["Duct"], `${at}.Duct`), `${at}.Duct`, ["width", "height"])) }; } if (row0["Pipe"] !== undefined) { bimModelArtifactGuardClosed(row0, `${at}`, ["Pipe"]); return { Pipe: ((row1) => ({ diameter: bimModelArtifactGuardNumber(row1["diameter"], `${at}.Pipe.diameter`) }))(bimModelArtifactGuardClosed(bimModelArtifactGuardObject(row0["Pipe"], `${at}.Pipe`), `${at}.Pipe`, ["diameter"])) }; } if (row0["Tray"] !== undefined) { bimModelArtifactGuardClosed(row0, `${at}`, ["Tray"]); return { Tray: ((row1) => ({ width: bimModelArtifactGuardNumber(row1["width"], `${at}.Tray.width`), height: bimModelArtifactGuardNumber(row1["height"], `${at}.Tray.height`) }))(bimModelArtifactGuardClosed(bimModelArtifactGuardObject(row0["Tray"], `${at}.Tray`), `${at}.Tray`, ["width", "height"])) }; } return bimModelArtifactGuardReject(`${at}`, "value matches no variant"); })(bimModelArtifactGuardObject(value, `${at}`));
}

export function parseMepSystem(value: unknown, at = "$"): MepSystem {
  return bimModelArtifactGuardMember(value, `${at}`, ["Supply", "Return", "Exhaust", "DomesticWater", "Waste", "Gas", "Power", "Data", "Lighting"] as const);
}

export function parseModelArtifact(value: unknown, at = "$"): ModelArtifact {
  const row = bimModelArtifactGuardClosed(bimModelArtifactGuardObject(value, at), at, ["schema", "project", "materials", "wall_types", "slab_types", "roof_types", "column_types", "beam_types", "window_types", "door_types", "curtain_wall_types", "sites", "buildings", "storeys", "grids", "walls", "curtain_walls", "curtain_panel_overrides", "columns", "beams", "slabs", "roofs", "openings", "stairs", "railings", "ramps", "spaces", "ceiling_types", "ceilings", "zones", "area_schemes", "space_conditions", "views", "sheets", "viewports", "sheet_revisions", "dimensions", "tags", "text_notes", "leaders", "annotation_styles", "families", "family_parameters", "family_solids", "components", "component_overrides", "mep_elements", "clash_sets", "rules", "issues", "issue_comments", "wall_sweeps", "schedules", "property_templates", "classification_systems", "properties", "classifications", "option_groups", "design_options", "worksets", "element_options", "element_worksets", "supports", "load_cases", "loads"]);
  return {
    schema: bimModelArtifactGuardString(row["schema"], `${at}.schema`),
    project: parseProject(row["project"], `${at}.project`),
    materials: row["materials"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelArtifactGuardObject(row["materials"], `${at}.materials`)).map(([key0, value0]) => [key0, parseMaterial(value0, `${at}.materials.${key0}`)])),
    wall_types: row["wall_types"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelArtifactGuardObject(row["wall_types"], `${at}.wall_types`)).map(([key0, value0]) => [key0, parseWallType(value0, `${at}.wall_types.${key0}`)])),
    slab_types: row["slab_types"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelArtifactGuardObject(row["slab_types"], `${at}.slab_types`)).map(([key0, value0]) => [key0, parseSlabType(value0, `${at}.slab_types.${key0}`)])),
    roof_types: row["roof_types"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelArtifactGuardObject(row["roof_types"], `${at}.roof_types`)).map(([key0, value0]) => [key0, parseRoofType(value0, `${at}.roof_types.${key0}`)])),
    column_types: row["column_types"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelArtifactGuardObject(row["column_types"], `${at}.column_types`)).map(([key0, value0]) => [key0, parseColumnType(value0, `${at}.column_types.${key0}`)])),
    beam_types: row["beam_types"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelArtifactGuardObject(row["beam_types"], `${at}.beam_types`)).map(([key0, value0]) => [key0, parseBeamType(value0, `${at}.beam_types.${key0}`)])),
    window_types: row["window_types"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelArtifactGuardObject(row["window_types"], `${at}.window_types`)).map(([key0, value0]) => [key0, parseWindowType(value0, `${at}.window_types.${key0}`)])),
    door_types: row["door_types"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelArtifactGuardObject(row["door_types"], `${at}.door_types`)).map(([key0, value0]) => [key0, parseDoorType(value0, `${at}.door_types.${key0}`)])),
    curtain_wall_types: row["curtain_wall_types"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelArtifactGuardObject(row["curtain_wall_types"], `${at}.curtain_wall_types`)).map(([key0, value0]) => [key0, parseCurtainWallType(value0, `${at}.curtain_wall_types.${key0}`)])),
    sites: row["sites"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelArtifactGuardObject(row["sites"], `${at}.sites`)).map(([key0, value0]) => [key0, parseSite(value0, `${at}.sites.${key0}`)])),
    buildings: row["buildings"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelArtifactGuardObject(row["buildings"], `${at}.buildings`)).map(([key0, value0]) => [key0, parseBuilding(value0, `${at}.buildings.${key0}`)])),
    storeys: row["storeys"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelArtifactGuardObject(row["storeys"], `${at}.storeys`)).map(([key0, value0]) => [key0, parseStorey(value0, `${at}.storeys.${key0}`)])),
    grids: row["grids"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelArtifactGuardObject(row["grids"], `${at}.grids`)).map(([key0, value0]) => [key0, parseGridLine(value0, `${at}.grids.${key0}`)])),
    walls: row["walls"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelArtifactGuardObject(row["walls"], `${at}.walls`)).map(([key0, value0]) => [key0, parseWall(value0, `${at}.walls.${key0}`)])),
    curtain_walls: row["curtain_walls"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelArtifactGuardObject(row["curtain_walls"], `${at}.curtain_walls`)).map(([key0, value0]) => [key0, parseCurtainWall(value0, `${at}.curtain_walls.${key0}`)])),
    curtain_panel_overrides: row["curtain_panel_overrides"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelArtifactGuardObject(row["curtain_panel_overrides"], `${at}.curtain_panel_overrides`)).map(([key0, value0]) => [key0, parseCurtainPanelOverride(value0, `${at}.curtain_panel_overrides.${key0}`)])),
    columns: row["columns"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelArtifactGuardObject(row["columns"], `${at}.columns`)).map(([key0, value0]) => [key0, parseColumn(value0, `${at}.columns.${key0}`)])),
    beams: row["beams"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelArtifactGuardObject(row["beams"], `${at}.beams`)).map(([key0, value0]) => [key0, parseBeam(value0, `${at}.beams.${key0}`)])),
    slabs: row["slabs"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelArtifactGuardObject(row["slabs"], `${at}.slabs`)).map(([key0, value0]) => [key0, parseSlab(value0, `${at}.slabs.${key0}`)])),
    roofs: row["roofs"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelArtifactGuardObject(row["roofs"], `${at}.roofs`)).map(([key0, value0]) => [key0, parseRoof(value0, `${at}.roofs.${key0}`)])),
    openings: row["openings"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelArtifactGuardObject(row["openings"], `${at}.openings`)).map(([key0, value0]) => [key0, parseOpening(value0, `${at}.openings.${key0}`)])),
    stairs: row["stairs"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelArtifactGuardObject(row["stairs"], `${at}.stairs`)).map(([key0, value0]) => [key0, parseStair(value0, `${at}.stairs.${key0}`)])),
    railings: row["railings"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelArtifactGuardObject(row["railings"], `${at}.railings`)).map(([key0, value0]) => [key0, parseRailing(value0, `${at}.railings.${key0}`)])),
    ramps: row["ramps"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelArtifactGuardObject(row["ramps"], `${at}.ramps`)).map(([key0, value0]) => [key0, parseRamp(value0, `${at}.ramps.${key0}`)])),
    spaces: row["spaces"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelArtifactGuardObject(row["spaces"], `${at}.spaces`)).map(([key0, value0]) => [key0, parseSpace(value0, `${at}.spaces.${key0}`)])),
    ceiling_types: row["ceiling_types"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelArtifactGuardObject(row["ceiling_types"], `${at}.ceiling_types`)).map(([key0, value0]) => [key0, parseCeilingType(value0, `${at}.ceiling_types.${key0}`)])),
    ceilings: row["ceilings"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelArtifactGuardObject(row["ceilings"], `${at}.ceilings`)).map(([key0, value0]) => [key0, parseCeiling(value0, `${at}.ceilings.${key0}`)])),
    zones: row["zones"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelArtifactGuardObject(row["zones"], `${at}.zones`)).map(([key0, value0]) => [key0, parseZone(value0, `${at}.zones.${key0}`)])),
    area_schemes: row["area_schemes"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelArtifactGuardObject(row["area_schemes"], `${at}.area_schemes`)).map(([key0, value0]) => [key0, parseAreaScheme(value0, `${at}.area_schemes.${key0}`)])),
    space_conditions: row["space_conditions"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelArtifactGuardObject(row["space_conditions"], `${at}.space_conditions`)).map(([key0, value0]) => [key0, parseSpaceConditions(value0, `${at}.space_conditions.${key0}`)])),
    views: row["views"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelArtifactGuardObject(row["views"], `${at}.views`)).map(([key0, value0]) => [key0, parseView(value0, `${at}.views.${key0}`)])),
    sheets: row["sheets"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelArtifactGuardObject(row["sheets"], `${at}.sheets`)).map(([key0, value0]) => [key0, parseSheet(value0, `${at}.sheets.${key0}`)])),
    viewports: row["viewports"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelArtifactGuardObject(row["viewports"], `${at}.viewports`)).map(([key0, value0]) => [key0, parseViewport(value0, `${at}.viewports.${key0}`)])),
    sheet_revisions: row["sheet_revisions"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelArtifactGuardObject(row["sheet_revisions"], `${at}.sheet_revisions`)).map(([key0, value0]) => [key0, parseSheetRevision(value0, `${at}.sheet_revisions.${key0}`)])),
    dimensions: row["dimensions"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelArtifactGuardObject(row["dimensions"], `${at}.dimensions`)).map(([key0, value0]) => [key0, parseDimension(value0, `${at}.dimensions.${key0}`)])),
    tags: row["tags"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelArtifactGuardObject(row["tags"], `${at}.tags`)).map(([key0, value0]) => [key0, parseTag(value0, `${at}.tags.${key0}`)])),
    text_notes: row["text_notes"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelArtifactGuardObject(row["text_notes"], `${at}.text_notes`)).map(([key0, value0]) => [key0, parseTextNote(value0, `${at}.text_notes.${key0}`)])),
    leaders: row["leaders"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelArtifactGuardObject(row["leaders"], `${at}.leaders`)).map(([key0, value0]) => [key0, parseLeader(value0, `${at}.leaders.${key0}`)])),
    annotation_styles: row["annotation_styles"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelArtifactGuardObject(row["annotation_styles"], `${at}.annotation_styles`)).map(([key0, value0]) => [key0, parseAnnotationStyle(value0, `${at}.annotation_styles.${key0}`)])),
    families: row["families"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelArtifactGuardObject(row["families"], `${at}.families`)).map(([key0, value0]) => [key0, parseFamily(value0, `${at}.families.${key0}`)])),
    family_parameters: row["family_parameters"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelArtifactGuardObject(row["family_parameters"], `${at}.family_parameters`)).map(([key0, value0]) => [key0, parseFamilyParameter(value0, `${at}.family_parameters.${key0}`)])),
    family_solids: row["family_solids"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelArtifactGuardObject(row["family_solids"], `${at}.family_solids`)).map(([key0, value0]) => [key0, parseFamilySolid(value0, `${at}.family_solids.${key0}`)])),
    components: row["components"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelArtifactGuardObject(row["components"], `${at}.components`)).map(([key0, value0]) => [key0, parseComponent(value0, `${at}.components.${key0}`)])),
    component_overrides: row["component_overrides"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelArtifactGuardObject(row["component_overrides"], `${at}.component_overrides`)).map(([key0, value0]) => [key0, parseComponentOverride(value0, `${at}.component_overrides.${key0}`)])),
    mep_elements: row["mep_elements"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelArtifactGuardObject(row["mep_elements"], `${at}.mep_elements`)).map(([key0, value0]) => [key0, parseMepElement(value0, `${at}.mep_elements.${key0}`)])),
    clash_sets: row["clash_sets"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelArtifactGuardObject(row["clash_sets"], `${at}.clash_sets`)).map(([key0, value0]) => [key0, parseClashSet(value0, `${at}.clash_sets.${key0}`)])),
    rules: row["rules"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelArtifactGuardObject(row["rules"], `${at}.rules`)).map(([key0, value0]) => [key0, parseRule(value0, `${at}.rules.${key0}`)])),
    issues: row["issues"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelArtifactGuardObject(row["issues"], `${at}.issues`)).map(([key0, value0]) => [key0, parseIssue(value0, `${at}.issues.${key0}`)])),
    issue_comments: row["issue_comments"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelArtifactGuardObject(row["issue_comments"], `${at}.issue_comments`)).map(([key0, value0]) => [key0, parseIssueComment(value0, `${at}.issue_comments.${key0}`)])),
    wall_sweeps: row["wall_sweeps"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelArtifactGuardObject(row["wall_sweeps"], `${at}.wall_sweeps`)).map(([key0, value0]) => [key0, parseWallSweep(value0, `${at}.wall_sweeps.${key0}`)])),
    schedules: row["schedules"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelArtifactGuardObject(row["schedules"], `${at}.schedules`)).map(([key0, value0]) => [key0, parseSchedule(value0, `${at}.schedules.${key0}`)])),
    property_templates: row["property_templates"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelArtifactGuardObject(row["property_templates"], `${at}.property_templates`)).map(([key0, value0]) => [key0, parsePropertyTemplate(value0, `${at}.property_templates.${key0}`)])),
    classification_systems: row["classification_systems"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelArtifactGuardObject(row["classification_systems"], `${at}.classification_systems`)).map(([key0, value0]) => [key0, parseClassificationSystem(value0, `${at}.classification_systems.${key0}`)])),
    properties: row["properties"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelArtifactGuardObject(row["properties"], `${at}.properties`)).map(([key0, value0]) => [key0, parsePropertySet(value0, `${at}.properties.${key0}`)])),
    classifications: row["classifications"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelArtifactGuardObject(row["classifications"], `${at}.classifications`)).map(([key0, value0]) => [key0, parseClassificationSet(value0, `${at}.classifications.${key0}`)])),
    option_groups: row["option_groups"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelArtifactGuardObject(row["option_groups"], `${at}.option_groups`)).map(([key0, value0]) => [key0, parseOptionGroup(value0, `${at}.option_groups.${key0}`)])),
    design_options: row["design_options"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelArtifactGuardObject(row["design_options"], `${at}.design_options`)).map(([key0, value0]) => [key0, parseDesignOption(value0, `${at}.design_options.${key0}`)])),
    worksets: row["worksets"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelArtifactGuardObject(row["worksets"], `${at}.worksets`)).map(([key0, value0]) => [key0, parseWorkset(value0, `${at}.worksets.${key0}`)])),
    element_options: row["element_options"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelArtifactGuardObject(row["element_options"], `${at}.element_options`)).map(([key0, value0]) => [key0, parseElementMembership(value0, `${at}.element_options.${key0}`)])),
    element_worksets: row["element_worksets"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelArtifactGuardObject(row["element_worksets"], `${at}.element_worksets`)).map(([key0, value0]) => [key0, parseElementMembership(value0, `${at}.element_worksets.${key0}`)])),
    supports: row["supports"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelArtifactGuardObject(row["supports"], `${at}.supports`)).map(([key0, value0]) => [key0, parseStructuralSupport(value0, `${at}.supports.${key0}`)])),
    load_cases: row["load_cases"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelArtifactGuardObject(row["load_cases"], `${at}.load_cases`)).map(([key0, value0]) => [key0, parseLoadCase(value0, `${at}.load_cases.${key0}`)])),
    loads: row["loads"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelArtifactGuardObject(row["loads"], `${at}.loads`)).map(([key0, value0]) => [key0, parseStructuralLoad(value0, `${at}.loads.${key0}`)])),
  };
}

export function parseOpening(value: unknown, at = "$"): Opening {
  const row = bimModelArtifactGuardClosed(bimModelArtifactGuardObject(value, at), at, ["host", "kind", "offset", "sill_override", "width", "height", "flip_hand", "flip_facing", "name", "reveal_depth", "reveal_material"]);
  return {
    host: bimModelArtifactGuardString(row["host"], `${at}.host`),
    kind: parseOpeningKind(row["kind"], `${at}.kind`),
    offset: bimModelArtifactGuardNumber(row["offset"], `${at}.offset`),
    sill_override: row["sill_override"] === undefined ? undefined : bimModelArtifactGuardNumber(row["sill_override"], `${at}.sill_override`),
    width: row["width"] === undefined ? undefined : bimModelArtifactGuardNumber(row["width"], `${at}.width`),
    height: row["height"] === undefined ? undefined : bimModelArtifactGuardNumber(row["height"], `${at}.height`),
    flip_hand: bimModelArtifactGuardBoolean(row["flip_hand"], `${at}.flip_hand`),
    flip_facing: bimModelArtifactGuardBoolean(row["flip_facing"], `${at}.flip_facing`),
    name: bimModelArtifactGuardString(row["name"], `${at}.name`),
    reveal_depth: row["reveal_depth"] === undefined ? undefined : bimModelArtifactGuardNumber(row["reveal_depth"], `${at}.reveal_depth`),
    reveal_material: row["reveal_material"] === undefined ? undefined : bimModelArtifactGuardString(row["reveal_material"], `${at}.reveal_material`),
  };
}

export function parseOpeningKind(value: unknown, at = "$"): OpeningKind {
  return ((row0) => { if (row0["Window"] !== undefined) { bimModelArtifactGuardClosed(row0, `${at}`, ["Window"]); return { Window: ((row1) => ({ window_type: bimModelArtifactGuardString(row1["window_type"], `${at}.Window.window_type`) }))(bimModelArtifactGuardClosed(bimModelArtifactGuardObject(row0["Window"], `${at}.Window`), `${at}.Window`, ["window_type"])) }; } if (row0["Door"] !== undefined) { bimModelArtifactGuardClosed(row0, `${at}`, ["Door"]); return { Door: ((row1) => ({ door_type: bimModelArtifactGuardString(row1["door_type"], `${at}.Door.door_type`) }))(bimModelArtifactGuardClosed(bimModelArtifactGuardObject(row0["Door"], `${at}.Door`), `${at}.Door`, ["door_type"])) }; } if (row0["Void"] !== undefined) { bimModelArtifactGuardClosed(row0, `${at}`, ["Void"]); return { Void: ((row1) => ({ width: bimModelArtifactGuardNumber(row1["width"], `${at}.Void.width`), height: bimModelArtifactGuardNumber(row1["height"], `${at}.Void.height`) }))(bimModelArtifactGuardClosed(bimModelArtifactGuardObject(row0["Void"], `${at}.Void`), `${at}.Void`, ["width", "height"])) }; } return bimModelArtifactGuardReject(`${at}`, "value matches no variant"); })(bimModelArtifactGuardObject(value, `${at}`));
}

export function parseOptionGroup(value: unknown, at = "$"): OptionGroup {
  const row = bimModelArtifactGuardClosed(bimModelArtifactGuardObject(value, at), at, ["name"]);
  return {
    name: bimModelArtifactGuardString(row["name"], `${at}.name`, {"minLength": 1}),
  };
}

export function parseOrientation(value: unknown, at = "$"): Orientation {
  return bimModelArtifactGuardMember(value, `${at}`, ["Landscape", "Portrait"] as const);
}

export function parsePaper(value: unknown, at = "$"): Paper {
  return ((row0) => { if (row0["Iso"] !== undefined) { bimModelArtifactGuardClosed(row0, `${at}`, ["Iso"]); return { Iso: ((row1) => ({ size: parseIsoSize(row1["size"], `${at}.Iso.size`) }))(bimModelArtifactGuardClosed(bimModelArtifactGuardObject(row0["Iso"], `${at}.Iso`), `${at}.Iso`, ["size"])) }; } if (row0["Custom"] !== undefined) { bimModelArtifactGuardClosed(row0, `${at}`, ["Custom"]); return { Custom: ((row1) => ({ width: bimModelArtifactGuardNumber(row1["width"], `${at}.Custom.width`), height: bimModelArtifactGuardNumber(row1["height"], `${at}.Custom.height`) }))(bimModelArtifactGuardClosed(bimModelArtifactGuardObject(row0["Custom"], `${at}.Custom`), `${at}.Custom`, ["width", "height"])) }; } return bimModelArtifactGuardReject(`${at}`, "value matches no variant"); })(bimModelArtifactGuardObject(value, `${at}`));
}

export function parseParameterKind(value: unknown, at = "$"): ParameterKind {
  return bimModelArtifactGuardMember(value, `${at}`, ["Length", "Angle", "Real", "Integer", "Boolean", "Text", "Material"] as const);
}

export function parseParametricProfile(value: unknown, at = "$"): ParametricProfile {
  return ((row0) => { if (row0["Rectangle"] !== undefined) { bimModelArtifactGuardClosed(row0, `${at}`, ["Rectangle"]); return { Rectangle: ((row1) => ({ width: bimModelArtifactGuardString(row1["width"], `${at}.Rectangle.width`), depth: bimModelArtifactGuardString(row1["depth"], `${at}.Rectangle.depth`) }))(bimModelArtifactGuardClosed(bimModelArtifactGuardObject(row0["Rectangle"], `${at}.Rectangle`), `${at}.Rectangle`, ["width", "depth"])) }; } if (row0["Circle"] !== undefined) { bimModelArtifactGuardClosed(row0, `${at}`, ["Circle"]); return { Circle: ((row1) => ({ diameter: bimModelArtifactGuardString(row1["diameter"], `${at}.Circle.diameter`) }))(bimModelArtifactGuardClosed(bimModelArtifactGuardObject(row0["Circle"], `${at}.Circle`), `${at}.Circle`, ["diameter"])) }; } if (row0["IShape"] !== undefined) { bimModelArtifactGuardClosed(row0, `${at}`, ["IShape"]); return { IShape: ((row1) => ({ width: bimModelArtifactGuardString(row1["width"], `${at}.IShape.width`), depth: bimModelArtifactGuardString(row1["depth"], `${at}.IShape.depth`), web: bimModelArtifactGuardString(row1["web"], `${at}.IShape.web`), flange: bimModelArtifactGuardString(row1["flange"], `${at}.IShape.flange`) }))(bimModelArtifactGuardClosed(bimModelArtifactGuardObject(row0["IShape"], `${at}.IShape`), `${at}.IShape`, ["width", "depth", "web", "flange"])) }; } if (row0["Polygon"] !== undefined) { bimModelArtifactGuardClosed(row0, `${at}`, ["Polygon"]); return { Polygon: ((row1) => ({ points: bimModelArtifactGuardArray(row1["points"], `${at}.Polygon.points`).map((item2, index2) => parseExprPoint(item2, `${at}.Polygon.points[${index2}]`)) }))(bimModelArtifactGuardClosed(bimModelArtifactGuardObject(row0["Polygon"], `${at}.Polygon`), `${at}.Polygon`, ["points"])) }; } return bimModelArtifactGuardReject(`${at}`, "value matches no variant"); })(bimModelArtifactGuardObject(value, `${at}`));
}

export function parsePhase(value: unknown, at = "$"): Phase {
  return bimModelArtifactGuardMember(value, `${at}`, ["Existing", "New", "Demolished", "Temporary"] as const);
}

export function parsePoint2(value: unknown, at = "$"): Point2 {
  const row = bimModelArtifactGuardClosed(bimModelArtifactGuardObject(value, at), at, ["x", "y"]);
  return {
    x: bimModelArtifactGuardNumber(row["x"], `${at}.x`),
    y: bimModelArtifactGuardNumber(row["y"], `${at}.y`),
  };
}

export function parsePoint3(value: unknown, at = "$"): Point3 {
  const row = bimModelArtifactGuardClosed(bimModelArtifactGuardObject(value, at), at, ["x", "y", "z"]);
  return {
    x: bimModelArtifactGuardNumber(row["x"], `${at}.x`),
    y: bimModelArtifactGuardNumber(row["y"], `${at}.y`),
    z: bimModelArtifactGuardNumber(row["z"], `${at}.z`),
  };
}

export interface PointLocation {
  readonly station: number;
}

export function parsePointLocation(value: unknown, at = "$"): PointLocation {
  const row = bimModelArtifactGuardClosed(bimModelArtifactGuardObject(value, at), at, ["station"]);
  return {
    station: bimModelArtifactGuardNumber(row["station"], `${at}.station`, {"minimum": 0, "maximum": 1}),
  };
}

export function parseProfile(value: unknown, at = "$"): Profile {
  return ((row0) => { if (row0["Rectangle"] !== undefined) { bimModelArtifactGuardClosed(row0, `${at}`, ["Rectangle"]); return { Rectangle: ((row1) => ({ width: bimModelArtifactGuardNumber(row1["width"], `${at}.Rectangle.width`), depth: bimModelArtifactGuardNumber(row1["depth"], `${at}.Rectangle.depth`) }))(bimModelArtifactGuardClosed(bimModelArtifactGuardObject(row0["Rectangle"], `${at}.Rectangle`), `${at}.Rectangle`, ["width", "depth"])) }; } if (row0["Circle"] !== undefined) { bimModelArtifactGuardClosed(row0, `${at}`, ["Circle"]); return { Circle: ((row1) => ({ diameter: bimModelArtifactGuardNumber(row1["diameter"], `${at}.Circle.diameter`) }))(bimModelArtifactGuardClosed(bimModelArtifactGuardObject(row0["Circle"], `${at}.Circle`), `${at}.Circle`, ["diameter"])) }; } if (row0["IShape"] !== undefined) { bimModelArtifactGuardClosed(row0, `${at}`, ["IShape"]); return { IShape: ((row1) => ({ width: bimModelArtifactGuardNumber(row1["width"], `${at}.IShape.width`), depth: bimModelArtifactGuardNumber(row1["depth"], `${at}.IShape.depth`), web: bimModelArtifactGuardNumber(row1["web"], `${at}.IShape.web`), flange: bimModelArtifactGuardNumber(row1["flange"], `${at}.IShape.flange`) }))(bimModelArtifactGuardClosed(bimModelArtifactGuardObject(row0["IShape"], `${at}.IShape`), `${at}.IShape`, ["width", "depth", "web", "flange"])) }; } if (row0["Custom"] !== undefined) { bimModelArtifactGuardClosed(row0, `${at}`, ["Custom"]); return { Custom: ((row1) => ({ outline: bimModelArtifactGuardArray(row1["outline"], `${at}.Custom.outline`).map((item2, index2) => parseVertex(item2, `${at}.Custom.outline[${index2}]`)) }))(bimModelArtifactGuardClosed(bimModelArtifactGuardObject(row0["Custom"], `${at}.Custom`), `${at}.Custom`, ["outline"])) }; } if (row0["Family"] !== undefined) { bimModelArtifactGuardClosed(row0, `${at}`, ["Family"]); return { Family: ((row1) => ({ family: bimModelArtifactGuardString(row1["family"], `${at}.Family.family`) }))(bimModelArtifactGuardClosed(bimModelArtifactGuardObject(row0["Family"], `${at}.Family`), `${at}.Family`, ["family"])) }; } return bimModelArtifactGuardReject(`${at}`, "value matches no variant"); })(bimModelArtifactGuardObject(value, `${at}`));
}

export function parseProject(value: unknown, at = "$"): Project {
  const row = bimModelArtifactGuardClosed(bimModelArtifactGuardObject(value, at), at, ["name", "description", "author", "organization", "phase_names"]);
  return {
    name: bimModelArtifactGuardString(row["name"], `${at}.name`),
    description: bimModelArtifactGuardString(row["description"], `${at}.description`),
    author: bimModelArtifactGuardString(row["author"], `${at}.author`),
    organization: bimModelArtifactGuardString(row["organization"], `${at}.organization`),
    phase_names: bimModelArtifactGuardArray(row["phase_names"], `${at}.phase_names`).map((item, index) => bimModelArtifactGuardString(item, `${at}.phase_names[${index}]`)),
  };
}

export function parsePropertyDef(value: unknown, at = "$"): PropertyDef {
  const row = bimModelArtifactGuardClosed(bimModelArtifactGuardObject(value, at), at, ["name", "kind", "unit", "description", "required", "default_value", "allowed", "minimum", "maximum"]);
  return {
    name: bimModelArtifactGuardString(row["name"], `${at}.name`),
    kind: parsePropertyKind(row["kind"], `${at}.kind`),
    unit: row["unit"] === undefined ? undefined : bimModelArtifactGuardString(row["unit"], `${at}.unit`),
    description: row["description"] === undefined ? undefined : bimModelArtifactGuardString(row["description"], `${at}.description`),
    required: bimModelArtifactGuardBoolean(row["required"], `${at}.required`),
    default_value: row["default_value"] === undefined ? undefined : parsePropertyValue(row["default_value"], `${at}.default_value`),
    allowed: bimModelArtifactGuardArray(row["allowed"], `${at}.allowed`).map((item, index) => parsePropertyValue(item, `${at}.allowed[${index}]`)),
    minimum: row["minimum"] === undefined ? undefined : bimModelArtifactGuardNumber(row["minimum"], `${at}.minimum`),
    maximum: row["maximum"] === undefined ? undefined : bimModelArtifactGuardNumber(row["maximum"], `${at}.maximum`),
  };
}

export function parsePropertyKind(value: unknown, at = "$"): PropertyKind {
  return bimModelArtifactGuardMember(value, `${at}`, ["Text", "Real", "Integer", "Boolean", "Length", "Area", "Volume", "Angle"] as const);
}

export function parsePropertySet(value: unknown, at = "$"): PropertySet {
  return Object.fromEntries(Object.entries(bimModelArtifactGuardObject(value, `${at}`)).map(([key0, value0]) => [key0, Object.fromEntries(Object.entries(bimModelArtifactGuardObject(value0, `${at}.${key0}`)).map(([key1, value1]) => [key1, parsePropertyValue(value1, `${at}.${key0}.${key1}`)]))]));
}

export function parsePropertyTemplate(value: unknown, at = "$"): PropertyTemplate {
  const row = bimModelArtifactGuardClosed(bimModelArtifactGuardObject(value, at), at, ["name", "applies_to", "properties"]);
  return {
    name: bimModelArtifactGuardString(row["name"], `${at}.name`),
    applies_to: bimModelArtifactGuardArray(row["applies_to"], `${at}.applies_to`).map((item, index) => parseTemplateTarget(item, `${at}.applies_to[${index}]`)),
    properties: bimModelArtifactGuardArray(row["properties"], `${at}.properties`).map((item, index) => parsePropertyDef(item, `${at}.properties[${index}]`)),
  };
}

export function parsePropertyValue(value: unknown, at = "$"): PropertyValue {
  return ((row0) => { if (row0["Text"] !== undefined) { bimModelArtifactGuardClosed(row0, `${at}`, ["Text"]); return { Text: ((row1) => ({ value: bimModelArtifactGuardString(row1["value"], `${at}.Text.value`) }))(bimModelArtifactGuardClosed(bimModelArtifactGuardObject(row0["Text"], `${at}.Text`), `${at}.Text`, ["value"])) }; } if (row0["Real"] !== undefined) { bimModelArtifactGuardClosed(row0, `${at}`, ["Real"]); return { Real: ((row1) => ({ value: bimModelArtifactGuardNumber(row1["value"], `${at}.Real.value`) }))(bimModelArtifactGuardClosed(bimModelArtifactGuardObject(row0["Real"], `${at}.Real`), `${at}.Real`, ["value"])) }; } if (row0["Integer"] !== undefined) { bimModelArtifactGuardClosed(row0, `${at}`, ["Integer"]); return { Integer: ((row1) => ({ value: bimModelArtifactGuardInteger(row1["value"], `${at}.Integer.value`, {"minimum": -2147483648, "maximum": 2147483647}) }))(bimModelArtifactGuardClosed(bimModelArtifactGuardObject(row0["Integer"], `${at}.Integer`), `${at}.Integer`, ["value"])) }; } if (row0["Boolean"] !== undefined) { bimModelArtifactGuardClosed(row0, `${at}`, ["Boolean"]); return { Boolean: ((row1) => ({ value: bimModelArtifactGuardBoolean(row1["value"], `${at}.Boolean.value`) }))(bimModelArtifactGuardClosed(bimModelArtifactGuardObject(row0["Boolean"], `${at}.Boolean`), `${at}.Boolean`, ["value"])) }; } if (row0["Length"] !== undefined) { bimModelArtifactGuardClosed(row0, `${at}`, ["Length"]); return { Length: ((row1) => ({ value: bimModelArtifactGuardNumber(row1["value"], `${at}.Length.value`) }))(bimModelArtifactGuardClosed(bimModelArtifactGuardObject(row0["Length"], `${at}.Length`), `${at}.Length`, ["value"])) }; } if (row0["Area"] !== undefined) { bimModelArtifactGuardClosed(row0, `${at}`, ["Area"]); return { Area: ((row1) => ({ value: bimModelArtifactGuardNumber(row1["value"], `${at}.Area.value`) }))(bimModelArtifactGuardClosed(bimModelArtifactGuardObject(row0["Area"], `${at}.Area`), `${at}.Area`, ["value"])) }; } if (row0["Volume"] !== undefined) { bimModelArtifactGuardClosed(row0, `${at}`, ["Volume"]); return { Volume: ((row1) => ({ value: bimModelArtifactGuardNumber(row1["value"], `${at}.Volume.value`) }))(bimModelArtifactGuardClosed(bimModelArtifactGuardObject(row0["Volume"], `${at}.Volume`), `${at}.Volume`, ["value"])) }; } if (row0["Angle"] !== undefined) { bimModelArtifactGuardClosed(row0, `${at}`, ["Angle"]); return { Angle: ((row1) => ({ value: bimModelArtifactGuardNumber(row1["value"], `${at}.Angle.value`) }))(bimModelArtifactGuardClosed(bimModelArtifactGuardObject(row0["Angle"], `${at}.Angle`), `${at}.Angle`, ["value"])) }; } return bimModelArtifactGuardReject(`${at}`, "value matches no variant"); })(bimModelArtifactGuardObject(value, `${at}`));
}

export function parseRailing(value: unknown, at = "$"): Railing {
  const row = bimModelArtifactGuardClosed(bimModelArtifactGuardObject(value, at), at, ["storey", "path", "height", "post_spacing", "profile", "post_profile", "baluster", "infill", "material", "base_offset", "host", "phase", "name"]);
  return {
    storey: bimModelArtifactGuardString(row["storey"], `${at}.storey`),
    path: bimModelArtifactGuardArray(row["path"], `${at}.path`).map((item, index) => parsePoint2(item, `${at}.path[${index}]`)),
    height: bimModelArtifactGuardNumber(row["height"], `${at}.height`),
    post_spacing: bimModelArtifactGuardNumber(row["post_spacing"], `${at}.post_spacing`),
    profile: parseProfile(row["profile"], `${at}.profile`),
    post_profile: parseProfile(row["post_profile"], `${at}.post_profile`),
    baluster: row["baluster"] === undefined ? undefined : parseBaluster(row["baluster"], `${at}.baluster`),
    infill: parseInfill(row["infill"], `${at}.infill`),
    material: bimModelArtifactGuardString(row["material"], `${at}.material`),
    base_offset: bimModelArtifactGuardNumber(row["base_offset"], `${at}.base_offset`),
    host: row["host"] === undefined ? undefined : parseRailingHost(row["host"], `${at}.host`),
    phase: parsePhase(row["phase"], `${at}.phase`),
    name: bimModelArtifactGuardString(row["name"], `${at}.name`),
  };
}

export function parseRailingHost(value: unknown, at = "$"): RailingHost {
  const row = bimModelArtifactGuardClosed(bimModelArtifactGuardObject(value, at), at, ["element", "side", "edge", "inset"]);
  return {
    element: bimModelArtifactGuardString(row["element"], `${at}.element`),
    side: parseHostSide(row["side"], `${at}.side`),
    edge: bimModelArtifactGuardInteger(row["edge"], `${at}.edge`, {"minimum": 0, "maximum": 4294967295}),
    inset: bimModelArtifactGuardNumber(row["inset"], `${at}.inset`),
  };
}

export function parseRamp(value: unknown, at = "$"): Ramp {
  const row = bimModelArtifactGuardClosed(bimModelArtifactGuardObject(value, at), at, ["storey", "path", "width", "landing_start", "landing_end", "landing_turn", "max_slope", "thickness", "material", "base_offset", "top", "railing_left", "railing_right", "name"]);
  return {
    storey: bimModelArtifactGuardString(row["storey"], `${at}.storey`),
    path: bimModelArtifactGuardArray(row["path"], `${at}.path`).map((item, index) => parseVertex(item, `${at}.path[${index}]`)),
    width: bimModelArtifactGuardNumber(row["width"], `${at}.width`),
    landing_start: bimModelArtifactGuardNumber(row["landing_start"], `${at}.landing_start`),
    landing_end: bimModelArtifactGuardNumber(row["landing_end"], `${at}.landing_end`),
    landing_turn: bimModelArtifactGuardNumber(row["landing_turn"], `${at}.landing_turn`),
    max_slope: bimModelArtifactGuardNumber(row["max_slope"], `${at}.max_slope`),
    thickness: bimModelArtifactGuardNumber(row["thickness"], `${at}.thickness`),
    material: bimModelArtifactGuardString(row["material"], `${at}.material`),
    base_offset: bimModelArtifactGuardNumber(row["base_offset"], `${at}.base_offset`),
    top: parseTopConstraint(row["top"], `${at}.top`),
    railing_left: bimModelArtifactGuardBoolean(row["railing_left"], `${at}.railing_left`),
    railing_right: bimModelArtifactGuardBoolean(row["railing_right"], `${at}.railing_right`),
    name: bimModelArtifactGuardString(row["name"], `${at}.name`),
  };
}

export function parseRestraints(value: unknown, at = "$"): Restraints {
  const row = bimModelArtifactGuardClosed(bimModelArtifactGuardObject(value, at), at, ["x", "y", "z", "rx", "ry", "rz"]);
  return {
    x: bimModelArtifactGuardBoolean(row["x"], `${at}.x`),
    y: bimModelArtifactGuardBoolean(row["y"], `${at}.y`),
    z: bimModelArtifactGuardBoolean(row["z"], `${at}.z`),
    rx: bimModelArtifactGuardBoolean(row["rx"], `${at}.rx`),
    ry: bimModelArtifactGuardBoolean(row["ry"], `${at}.ry`),
    rz: bimModelArtifactGuardBoolean(row["rz"], `${at}.rz`),
  };
}

export function parseRgb(value: unknown, at = "$"): Rgb {
  const row = bimModelArtifactGuardClosed(bimModelArtifactGuardObject(value, at), at, ["r", "g", "b"]);
  return {
    r: bimModelArtifactGuardNumber(row["r"], `${at}.r`),
    g: bimModelArtifactGuardNumber(row["g"], `${at}.g`),
    b: bimModelArtifactGuardNumber(row["b"], `${at}.b`),
  };
}

export function parseRiserKind(value: unknown, at = "$"): RiserKind {
  return bimModelArtifactGuardMember(value, `${at}`, ["Open", "Closed"] as const);
}

export function parseRoof(value: unknown, at = "$"): Roof {
  const row = bimModelArtifactGuardClosed(bimModelArtifactGuardObject(value, at), at, ["storey", "roof_type", "footprint", "shape", "overhang", "base_offset", "phase", "name"]);
  return {
    storey: bimModelArtifactGuardString(row["storey"], `${at}.storey`),
    roof_type: bimModelArtifactGuardString(row["roof_type"], `${at}.roof_type`),
    footprint: bimModelArtifactGuardArray(row["footprint"], `${at}.footprint`).map((item, index) => parseVertex(item, `${at}.footprint[${index}]`)),
    shape: parseRoofShape(row["shape"], `${at}.shape`),
    overhang: bimModelArtifactGuardNumber(row["overhang"], `${at}.overhang`),
    base_offset: bimModelArtifactGuardNumber(row["base_offset"], `${at}.base_offset`),
    phase: parsePhase(row["phase"], `${at}.phase`),
    name: bimModelArtifactGuardString(row["name"], `${at}.name`),
  };
}

export function parseRoofShape(value: unknown, at = "$"): RoofShape {
  return ((input0: unknown) => { if (typeof input0 === "string") { try { return bimModelArtifactGuardConstant(input0, `${at}`, "Flat"); } catch (error) { if (!(error instanceof bimModelArtifactGuardRefusal)) throw error; } } if (typeof input0 === "object" && input0 !== null && !Array.isArray(input0)) { try { return ((row1) => ({ Shed: ((row2) => ({ pitch: bimModelArtifactGuardNumber(row2["pitch"], `${at}.Shed.pitch`), direction: bimModelArtifactGuardNumber(row2["direction"], `${at}.Shed.direction`) }))(bimModelArtifactGuardClosed(bimModelArtifactGuardObject(row1["Shed"], `${at}.Shed`), `${at}.Shed`, ["pitch", "direction"])) }))(bimModelArtifactGuardClosed(bimModelArtifactGuardObject(input0, `${at}`), `${at}`, ["Shed"])); } catch (error) { if (!(error instanceof bimModelArtifactGuardRefusal)) throw error; } } if (typeof input0 === "object" && input0 !== null && !Array.isArray(input0)) { try { return ((row1) => ({ Gable: ((row2) => ({ pitch: bimModelArtifactGuardNumber(row2["pitch"], `${at}.Gable.pitch`), ridge_direction: bimModelArtifactGuardNumber(row2["ridge_direction"], `${at}.Gable.ridge_direction`) }))(bimModelArtifactGuardClosed(bimModelArtifactGuardObject(row1["Gable"], `${at}.Gable`), `${at}.Gable`, ["pitch", "ridge_direction"])) }))(bimModelArtifactGuardClosed(bimModelArtifactGuardObject(input0, `${at}`), `${at}`, ["Gable"])); } catch (error) { if (!(error instanceof bimModelArtifactGuardRefusal)) throw error; } } if (typeof input0 === "object" && input0 !== null && !Array.isArray(input0)) { try { return ((row1) => ({ Hip: ((row2) => ({ pitch: bimModelArtifactGuardNumber(row2["pitch"], `${at}.Hip.pitch`) }))(bimModelArtifactGuardClosed(bimModelArtifactGuardObject(row1["Hip"], `${at}.Hip`), `${at}.Hip`, ["pitch"])) }))(bimModelArtifactGuardClosed(bimModelArtifactGuardObject(input0, `${at}`), `${at}`, ["Hip"])); } catch (error) { if (!(error instanceof bimModelArtifactGuardRefusal)) throw error; } } if (typeof input0 === "object" && input0 !== null && !Array.isArray(input0)) { try { return ((row1) => ({ Mansard: ((row2) => ({ lower_pitch: bimModelArtifactGuardNumber(row2["lower_pitch"], `${at}.Mansard.lower_pitch`), upper_pitch: bimModelArtifactGuardNumber(row2["upper_pitch"], `${at}.Mansard.upper_pitch`), break_height: bimModelArtifactGuardNumber(row2["break_height"], `${at}.Mansard.break_height`) }))(bimModelArtifactGuardClosed(bimModelArtifactGuardObject(row1["Mansard"], `${at}.Mansard`), `${at}.Mansard`, ["lower_pitch", "upper_pitch", "break_height"])) }))(bimModelArtifactGuardClosed(bimModelArtifactGuardObject(input0, `${at}`), `${at}`, ["Mansard"])); } catch (error) { if (!(error instanceof bimModelArtifactGuardRefusal)) throw error; } } return bimModelArtifactGuardReject(`${at}`, "value matches no variant"); })(value);
}

export function parseRoofType(value: unknown, at = "$"): RoofType {
  const row = bimModelArtifactGuardClosed(bimModelArtifactGuardObject(value, at), at, ["name", "layers"]);
  return {
    name: bimModelArtifactGuardString(row["name"], `${at}.name`),
    layers: bimModelArtifactGuardArray(row["layers"], `${at}.layers`).map((item, index) => parseLayer(item, `${at}.layers[${index}]`)),
  };
}

export function parseRule(value: unknown, at = "$"): Rule {
  const row = bimModelArtifactGuardClosed(bimModelArtifactGuardObject(value, at), at, ["name", "kind", "limit", "severity", "scope"]);
  return {
    name: bimModelArtifactGuardString(row["name"], `${at}.name`),
    kind: parseRuleKind(row["kind"], `${at}.kind`),
    limit: bimModelArtifactGuardNumber(row["limit"], `${at}.limit`),
    severity: parseRuleSeverity(row["severity"], `${at}.severity`),
    scope: parseRuleScope(row["scope"], `${at}.scope`),
  };
}

export function parseRuleKind(value: unknown, at = "$"): RuleKind {
  return bimModelArtifactGuardMember(value, `${at}`, ["MinClearHeight", "MaxRiser", "MinTread", "MinStairWidth", "MinDoorWidth", "MaxRampSlope", "MinCorridorWidth", "MaxCompartmentArea"] as const);
}

export function parseRuleScope(value: unknown, at = "$"): RuleScope {
  const row = bimModelArtifactGuardClosed(bimModelArtifactGuardObject(value, at), at, ["storeys", "phases", "ids", "filter"]);
  return {
    storeys: bimModelArtifactGuardArray(row["storeys"], `${at}.storeys`).map((item, index) => bimModelArtifactGuardString(item, `${at}.storeys[${index}]`)),
    phases: bimModelArtifactGuardArray(row["phases"], `${at}.phases`).map((item, index) => parsePhase(item, `${at}.phases[${index}]`)),
    ids: bimModelArtifactGuardArray(row["ids"], `${at}.ids`).map((item, index) => bimModelArtifactGuardString(item, `${at}.ids[${index}]`)),
    filter: bimModelArtifactGuardString(row["filter"], `${at}.filter`),
  };
}

export function parseRuleSeverity(value: unknown, at = "$"): RuleSeverity {
  return bimModelArtifactGuardMember(value, `${at}`, ["Error", "Warning", "Note"] as const);
}

export function parseSchedule(value: unknown, at = "$"): Schedule {
  const row = bimModelArtifactGuardClosed(bimModelArtifactGuardObject(value, at), at, ["name", "category", "columns", "sort", "filter", "group", "itemize", "storeys", "phases"]);
  return {
    name: bimModelArtifactGuardString(row["name"], `${at}.name`),
    category: parseScheduleCategory(row["category"], `${at}.category`),
    columns: bimModelArtifactGuardArray(row["columns"], `${at}.columns`).map((item, index) => parseScheduleColumn(item, `${at}.columns[${index}]`)),
    sort: bimModelArtifactGuardArray(row["sort"], `${at}.sort`).map((item, index) => parseScheduleSort(item, `${at}.sort[${index}]`)),
    filter: bimModelArtifactGuardArray(row["filter"], `${at}.filter`).map((item, index) => parseScheduleFilter(item, `${at}.filter[${index}]`)),
    group: bimModelArtifactGuardArray(row["group"], `${at}.group`).map((item, index) => parseScheduleGroup(item, `${at}.group[${index}]`)),
    itemize: bimModelArtifactGuardBoolean(row["itemize"], `${at}.itemize`),
    storeys: bimModelArtifactGuardArray(row["storeys"], `${at}.storeys`).map((item, index) => bimModelArtifactGuardString(item, `${at}.storeys[${index}]`)),
    phases: bimModelArtifactGuardArray(row["phases"], `${at}.phases`).map((item, index) => parsePhase(item, `${at}.phases[${index}]`)),
  };
}

export function parseScheduleCategory(value: unknown, at = "$"): ScheduleCategory {
  return bimModelArtifactGuardMember(value, `${at}`, ["Wall", "CurtainWall", "Slab", "Roof", "Column", "Beam", "Window", "Door", "Void", "Stair", "Railing", "Space", "Finish", "Material"] as const);
}

export function parseScheduleColumn(value: unknown, at = "$"): ScheduleColumn {
  const row = bimModelArtifactGuardClosed(bimModelArtifactGuardObject(value, at), at, ["key", "heading", "total"]);
  return {
    key: parseScheduleKey(row["key"], `${at}.key`),
    heading: row["heading"] === undefined ? undefined : bimModelArtifactGuardString(row["heading"], `${at}.heading`),
    total: bimModelArtifactGuardBoolean(row["total"], `${at}.total`),
  };
}

export function parseScheduleField(value: unknown, at = "$"): ScheduleField {
  return bimModelArtifactGuardMember(value, `${at}`, ["Id", "Name", "Kind", "Storey", "Level", "Type", "Phase", "Material", "Host", "Number", "Usage", "Surface", "Swing", "Leaves", "Panes", "Count", "Length", "Width", "Height", "Perimeter", "GrossSideArea", "OpeningArea", "NetSideArea", "GrossArea", "NetArea", "SurfaceArea", "GrossVolume", "NetVolume", "Mass", "Risers", "Thickness", "LayerArea", "LayerVolume", "LayerMass", "FinishArea", "UValue", "GValue"] as const);
}

export function parseScheduleFilter(value: unknown, at = "$"): ScheduleFilter {
  const row = bimModelArtifactGuardClosed(bimModelArtifactGuardObject(value, at), at, ["key", "op", "value"]);
  return {
    key: parseScheduleKey(row["key"], `${at}.key`),
    op: parseScheduleOp(row["op"], `${at}.op`),
    value: bimModelArtifactGuardString(row["value"], `${at}.value`),
  };
}

export function parseScheduleGroup(value: unknown, at = "$"): ScheduleGroup {
  const row = bimModelArtifactGuardClosed(bimModelArtifactGuardObject(value, at), at, ["key"]);
  return {
    key: parseScheduleKey(row["key"], `${at}.key`),
  };
}

export function parseScheduleKey(value: unknown, at = "$"): ScheduleKey {
  return ((row0) => { if (row0["Field"] !== undefined) { bimModelArtifactGuardClosed(row0, `${at}`, ["Field"]); return { Field: ((row1) => ({ field: parseScheduleField(row1["field"], `${at}.Field.field`) }))(bimModelArtifactGuardClosed(bimModelArtifactGuardObject(row0["Field"], `${at}.Field`), `${at}.Field`, ["field"])) }; } if (row0["Property"] !== undefined) { bimModelArtifactGuardClosed(row0, `${at}`, ["Property"]); return { Property: ((row1) => ({ set: bimModelArtifactGuardString(row1["set"], `${at}.Property.set`), name: bimModelArtifactGuardString(row1["name"], `${at}.Property.name`) }))(bimModelArtifactGuardClosed(bimModelArtifactGuardObject(row0["Property"], `${at}.Property`), `${at}.Property`, ["set", "name"])) }; } return bimModelArtifactGuardReject(`${at}`, "value matches no variant"); })(bimModelArtifactGuardObject(value, `${at}`));
}

export function parseScheduleOp(value: unknown, at = "$"): ScheduleOp {
  return bimModelArtifactGuardMember(value, `${at}`, ["Equals", "NotEquals", "Contains", "Greater", "GreaterOrEqual", "Less", "LessOrEqual", "Empty", "NotEmpty"] as const);
}

export function parseScheduleSort(value: unknown, at = "$"): ScheduleSort {
  const row = bimModelArtifactGuardClosed(bimModelArtifactGuardObject(value, at), at, ["key", "descending"]);
  return {
    key: parseScheduleKey(row["key"], `${at}.key`),
    descending: bimModelArtifactGuardBoolean(row["descending"], `${at}.descending`),
  };
}

export function parseSectionBox(value: unknown, at = "$"): SectionBox {
  const row = bimModelArtifactGuardClosed(bimModelArtifactGuardObject(value, at), at, ["min", "max"]);
  return {
    min: parsePoint3(row["min"], `${at}.min`),
    max: parsePoint3(row["max"], `${at}.max`),
  };
}

export function parseSheet(value: unknown, at = "$"): Sheet {
  const row = bimModelArtifactGuardClosed(bimModelArtifactGuardObject(value, at), at, ["number", "name", "paper", "orientation", "project", "drawn_by", "checked_by", "date", "revision", "scale_label"]);
  return {
    number: bimModelArtifactGuardString(row["number"], `${at}.number`),
    name: bimModelArtifactGuardString(row["name"], `${at}.name`),
    paper: parsePaper(row["paper"], `${at}.paper`),
    orientation: parseOrientation(row["orientation"], `${at}.orientation`),
    project: bimModelArtifactGuardString(row["project"], `${at}.project`),
    drawn_by: bimModelArtifactGuardString(row["drawn_by"], `${at}.drawn_by`),
    checked_by: bimModelArtifactGuardString(row["checked_by"], `${at}.checked_by`),
    date: bimModelArtifactGuardString(row["date"], `${at}.date`),
    revision: bimModelArtifactGuardString(row["revision"], `${at}.revision`),
    scale_label: bimModelArtifactGuardString(row["scale_label"], `${at}.scale_label`),
  };
}

export function parseSheetRevision(value: unknown, at = "$"): SheetRevision {
  const row = bimModelArtifactGuardClosed(bimModelArtifactGuardObject(value, at), at, ["sheet", "number", "date", "description", "author"]);
  return {
    sheet: bimModelArtifactGuardString(row["sheet"], `${at}.sheet`),
    number: bimModelArtifactGuardString(row["number"], `${at}.number`),
    date: bimModelArtifactGuardString(row["date"], `${at}.date`),
    description: bimModelArtifactGuardString(row["description"], `${at}.description`),
    author: bimModelArtifactGuardString(row["author"], `${at}.author`),
  };
}

export function parseSite(value: unknown, at = "$"): Site {
  const row = bimModelArtifactGuardClosed(bimModelArtifactGuardObject(value, at), at, ["name", "latitude", "longitude", "elevation", "true_north", "boundary"]);
  return {
    name: bimModelArtifactGuardString(row["name"], `${at}.name`),
    latitude: bimModelArtifactGuardNumber(row["latitude"], `${at}.latitude`),
    longitude: bimModelArtifactGuardNumber(row["longitude"], `${at}.longitude`),
    elevation: bimModelArtifactGuardNumber(row["elevation"], `${at}.elevation`),
    true_north: bimModelArtifactGuardNumber(row["true_north"], `${at}.true_north`),
    boundary: bimModelArtifactGuardArray(row["boundary"], `${at}.boundary`).map((item, index) => parsePoint2(item, `${at}.boundary[${index}]`)),
  };
}

export function parseSlab(value: unknown, at = "$"): Slab {
  const row = bimModelArtifactGuardClosed(bimModelArtifactGuardObject(value, at), at, ["storey", "slab_type", "boundary", "holes", "offset", "slope", "phase", "name"]);
  return {
    storey: bimModelArtifactGuardString(row["storey"], `${at}.storey`),
    slab_type: bimModelArtifactGuardString(row["slab_type"], `${at}.slab_type`),
    boundary: bimModelArtifactGuardArray(row["boundary"], `${at}.boundary`).map((item, index) => parseVertex(item, `${at}.boundary[${index}]`)),
    holes: bimModelArtifactGuardArray(row["holes"], `${at}.holes`).map((item, index) => bimModelArtifactGuardArray(item, `${at}.holes[${index}]`).map((item1, index1) => parseVertex(item1, `${at}.holes[${index}][${index1}]`))),
    offset: bimModelArtifactGuardNumber(row["offset"], `${at}.offset`),
    slope: row["slope"] === undefined ? undefined : parseSlope(row["slope"], `${at}.slope`),
    phase: parsePhase(row["phase"], `${at}.phase`),
    name: bimModelArtifactGuardString(row["name"], `${at}.name`),
  };
}

export function parseSlabType(value: unknown, at = "$"): SlabType {
  const row = bimModelArtifactGuardClosed(bimModelArtifactGuardObject(value, at), at, ["name", "layers"]);
  return {
    name: bimModelArtifactGuardString(row["name"], `${at}.name`),
    layers: bimModelArtifactGuardArray(row["layers"], `${at}.layers`).map((item, index) => parseLayer(item, `${at}.layers[${index}]`)),
  };
}

export function parseSlope(value: unknown, at = "$"): Slope {
  const row = bimModelArtifactGuardClosed(bimModelArtifactGuardObject(value, at), at, ["direction", "angle"]);
  return {
    direction: bimModelArtifactGuardNumber(row["direction"], `${at}.direction`),
    angle: bimModelArtifactGuardNumber(row["angle"], `${at}.angle`),
  };
}

export function parseSolidAxis(value: unknown, at = "$"): SolidAxis {
  return bimModelArtifactGuardMember(value, `${at}`, ["X", "Y", "Z"] as const);
}

export function parseSolidShape(value: unknown, at = "$"): SolidShape {
  return ((row0) => { if (row0["Extrusion"] !== undefined) { bimModelArtifactGuardClosed(row0, `${at}`, ["Extrusion"]); return { Extrusion: ((row1) => ({ profile: parseParametricProfile(row1["profile"], `${at}.Extrusion.profile`), base: bimModelArtifactGuardString(row1["base"], `${at}.Extrusion.base`), height: bimModelArtifactGuardString(row1["height"], `${at}.Extrusion.height`) }))(bimModelArtifactGuardClosed(bimModelArtifactGuardObject(row0["Extrusion"], `${at}.Extrusion`), `${at}.Extrusion`, ["profile", "base", "height"])) }; } if (row0["Revolution"] !== undefined) { bimModelArtifactGuardClosed(row0, `${at}`, ["Revolution"]); return { Revolution: ((row1) => ({ profile: parseParametricProfile(row1["profile"], `${at}.Revolution.profile`), axis: parseSolidAxis(row1["axis"], `${at}.Revolution.axis`), angle: bimModelArtifactGuardString(row1["angle"], `${at}.Revolution.angle`) }))(bimModelArtifactGuardClosed(bimModelArtifactGuardObject(row0["Revolution"], `${at}.Revolution`), `${at}.Revolution`, ["profile", "axis", "angle"])) }; } if (row0["Sweep"] !== undefined) { bimModelArtifactGuardClosed(row0, `${at}`, ["Sweep"]); return { Sweep: ((row1) => ({ profile: parseParametricProfile(row1["profile"], `${at}.Sweep.profile`), path: bimModelArtifactGuardArray(row1["path"], `${at}.Sweep.path`).map((item2, index2) => parseExprPoint(item2, `${at}.Sweep.path[${index2}]`)) }))(bimModelArtifactGuardClosed(bimModelArtifactGuardObject(row0["Sweep"], `${at}.Sweep`), `${at}.Sweep`, ["profile", "path"])) }; } if (row0["Cuboid"] !== undefined) { bimModelArtifactGuardClosed(row0, `${at}`, ["Cuboid"]); return { Cuboid: ((row1) => ({ x: bimModelArtifactGuardString(row1["x"], `${at}.Cuboid.x`), y: bimModelArtifactGuardString(row1["y"], `${at}.Cuboid.y`), z: bimModelArtifactGuardString(row1["z"], `${at}.Cuboid.z`), width: bimModelArtifactGuardString(row1["width"], `${at}.Cuboid.width`), depth: bimModelArtifactGuardString(row1["depth"], `${at}.Cuboid.depth`), height: bimModelArtifactGuardString(row1["height"], `${at}.Cuboid.height`) }))(bimModelArtifactGuardClosed(bimModelArtifactGuardObject(row0["Cuboid"], `${at}.Cuboid`), `${at}.Cuboid`, ["x", "y", "z", "width", "depth", "height"])) }; } return bimModelArtifactGuardReject(`${at}`, "value matches no variant"); })(bimModelArtifactGuardObject(value, `${at}`));
}

export function parseSpace(value: unknown, at = "$"): Space {
  const row = bimModelArtifactGuardClosed(bimModelArtifactGuardObject(value, at), at, ["storey", "number", "name", "boundary", "usage", "phase", "zone", "floor_finish", "wall_finish", "ceiling_finish"]);
  return {
    storey: bimModelArtifactGuardString(row["storey"], `${at}.storey`),
    number: bimModelArtifactGuardString(row["number"], `${at}.number`),
    name: bimModelArtifactGuardString(row["name"], `${at}.name`),
    boundary: parseSpaceBoundary(row["boundary"], `${at}.boundary`),
    usage: bimModelArtifactGuardString(row["usage"], `${at}.usage`),
    phase: parsePhase(row["phase"], `${at}.phase`),
    zone: row["zone"] === undefined ? undefined : bimModelArtifactGuardString(row["zone"], `${at}.zone`),
    floor_finish: row["floor_finish"] === undefined ? undefined : bimModelArtifactGuardString(row["floor_finish"], `${at}.floor_finish`),
    wall_finish: row["wall_finish"] === undefined ? undefined : bimModelArtifactGuardString(row["wall_finish"], `${at}.wall_finish`),
    ceiling_finish: row["ceiling_finish"] === undefined ? undefined : bimModelArtifactGuardString(row["ceiling_finish"], `${at}.ceiling_finish`),
  };
}

export function parseSpaceBoundary(value: unknown, at = "$"): SpaceBoundary {
  return ((row0) => { if (row0["Bounded"] !== undefined) { bimModelArtifactGuardClosed(row0, `${at}`, ["Bounded"]); return { Bounded: ((row1) => ({ seed: parsePoint2(row1["seed"], `${at}.Bounded.seed`) }))(bimModelArtifactGuardClosed(bimModelArtifactGuardObject(row0["Bounded"], `${at}.Bounded`), `${at}.Bounded`, ["seed"])) }; } if (row0["Explicit"] !== undefined) { bimModelArtifactGuardClosed(row0, `${at}`, ["Explicit"]); return { Explicit: ((row1) => ({ outline: bimModelArtifactGuardArray(row1["outline"], `${at}.Explicit.outline`).map((item2, index2) => parseVertex(item2, `${at}.Explicit.outline[${index2}]`)) }))(bimModelArtifactGuardClosed(bimModelArtifactGuardObject(row0["Explicit"], `${at}.Explicit`), `${at}.Explicit`, ["outline"])) }; } return bimModelArtifactGuardReject(`${at}`, "value matches no variant"); })(bimModelArtifactGuardObject(value, `${at}`));
}

export function parseSpaceConditions(value: unknown, at = "$"): SpaceConditions {
  const row = bimModelArtifactGuardClosed(bimModelArtifactGuardObject(value, at), at, ["occupancy", "occupancy_density", "heating_setpoint", "cooling_setpoint", "ventilation_rate", "lighting_power_density", "equipment_power_density", "schedule"]);
  return {
    occupancy: row["occupancy"] === undefined ? undefined : bimModelArtifactGuardString(row["occupancy"], `${at}.occupancy`),
    occupancy_density: row["occupancy_density"] === undefined ? undefined : bimModelArtifactGuardNumber(row["occupancy_density"], `${at}.occupancy_density`),
    heating_setpoint: row["heating_setpoint"] === undefined ? undefined : bimModelArtifactGuardNumber(row["heating_setpoint"], `${at}.heating_setpoint`),
    cooling_setpoint: row["cooling_setpoint"] === undefined ? undefined : bimModelArtifactGuardNumber(row["cooling_setpoint"], `${at}.cooling_setpoint`),
    ventilation_rate: row["ventilation_rate"] === undefined ? undefined : bimModelArtifactGuardNumber(row["ventilation_rate"], `${at}.ventilation_rate`),
    lighting_power_density: row["lighting_power_density"] === undefined ? undefined : bimModelArtifactGuardNumber(row["lighting_power_density"], `${at}.lighting_power_density`),
    equipment_power_density: row["equipment_power_density"] === undefined ? undefined : bimModelArtifactGuardNumber(row["equipment_power_density"], `${at}.equipment_power_density`),
    schedule: row["schedule"] === undefined ? undefined : bimModelArtifactGuardString(row["schedule"], `${at}.schedule`),
  };
}

export function parseStair(value: unknown, at = "$"): Stair {
  const row = bimModelArtifactGuardClosed(bimModelArtifactGuardObject(value, at), at, ["storey", "start", "direction", "width", "flight", "top", "max_riser", "min_tread", "stringer", "nosing", "tread_thickness", "riser", "landing_depth", "phase", "name"]);
  return {
    storey: bimModelArtifactGuardString(row["storey"], `${at}.storey`),
    start: parsePoint2(row["start"], `${at}.start`),
    direction: bimModelArtifactGuardNumber(row["direction"], `${at}.direction`),
    width: bimModelArtifactGuardNumber(row["width"], `${at}.width`),
    flight: parseStairFlight(row["flight"], `${at}.flight`),
    top: parseTopConstraint(row["top"], `${at}.top`),
    max_riser: bimModelArtifactGuardNumber(row["max_riser"], `${at}.max_riser`),
    min_tread: bimModelArtifactGuardNumber(row["min_tread"], `${at}.min_tread`),
    stringer: parseStairStringer(row["stringer"], `${at}.stringer`),
    nosing: bimModelArtifactGuardNumber(row["nosing"], `${at}.nosing`),
    tread_thickness: bimModelArtifactGuardNumber(row["tread_thickness"], `${at}.tread_thickness`),
    riser: parseRiserKind(row["riser"], `${at}.riser`),
    landing_depth: bimModelArtifactGuardNumber(row["landing_depth"], `${at}.landing_depth`),
    phase: parsePhase(row["phase"], `${at}.phase`),
    name: bimModelArtifactGuardString(row["name"], `${at}.name`),
  };
}

export function parseStairFlight(value: unknown, at = "$"): StairFlight {
  return ((input0: unknown) => { if (typeof input0 === "string") { try { return bimModelArtifactGuardConstant(input0, `${at}`, "Straight"); } catch (error) { if (!(error instanceof bimModelArtifactGuardRefusal)) throw error; } } if (typeof input0 === "object" && input0 !== null && !Array.isArray(input0)) { try { return ((row1) => ({ LTurn: ((row2) => ({ split: bimModelArtifactGuardNumber(row2["split"], `${at}.LTurn.split`), turn: parseTurn(row2["turn"], `${at}.LTurn.turn`) }))(bimModelArtifactGuardClosed(bimModelArtifactGuardObject(row1["LTurn"], `${at}.LTurn`), `${at}.LTurn`, ["split", "turn"])) }))(bimModelArtifactGuardClosed(bimModelArtifactGuardObject(input0, `${at}`), `${at}`, ["LTurn"])); } catch (error) { if (!(error instanceof bimModelArtifactGuardRefusal)) throw error; } } if (typeof input0 === "object" && input0 !== null && !Array.isArray(input0)) { try { return ((row1) => ({ UTurn: ((row2) => ({ gap: bimModelArtifactGuardNumber(row2["gap"], `${at}.UTurn.gap`) }))(bimModelArtifactGuardClosed(bimModelArtifactGuardObject(row1["UTurn"], `${at}.UTurn`), `${at}.UTurn`, ["gap"])) }))(bimModelArtifactGuardClosed(bimModelArtifactGuardObject(input0, `${at}`), `${at}`, ["UTurn"])); } catch (error) { if (!(error instanceof bimModelArtifactGuardRefusal)) throw error; } } if (typeof input0 === "object" && input0 !== null && !Array.isArray(input0)) { try { return ((row1) => ({ Spiral: ((row2) => ({ radius: bimModelArtifactGuardNumber(row2["radius"], `${at}.Spiral.radius`), sweep: bimModelArtifactGuardNumber(row2["sweep"], `${at}.Spiral.sweep`) }))(bimModelArtifactGuardClosed(bimModelArtifactGuardObject(row1["Spiral"], `${at}.Spiral`), `${at}.Spiral`, ["radius", "sweep"])) }))(bimModelArtifactGuardClosed(bimModelArtifactGuardObject(input0, `${at}`), `${at}`, ["Spiral"])); } catch (error) { if (!(error instanceof bimModelArtifactGuardRefusal)) throw error; } } return bimModelArtifactGuardReject(`${at}`, "value matches no variant"); })(value);
}

export function parseStairStringer(value: unknown, at = "$"): StairStringer {
  const row = bimModelArtifactGuardClosed(bimModelArtifactGuardObject(value, at), at, ["kind", "width", "depth"]);
  return {
    kind: parseStringerKind(row["kind"], `${at}.kind`),
    width: bimModelArtifactGuardNumber(row["width"], `${at}.width`),
    depth: bimModelArtifactGuardNumber(row["depth"], `${at}.depth`),
  };
}

export function parseStorey(value: unknown, at = "$"): Storey {
  const row = bimModelArtifactGuardClosed(bimModelArtifactGuardObject(value, at), at, ["building", "name", "level", "height", "cut_height"]);
  return {
    building: bimModelArtifactGuardString(row["building"], `${at}.building`),
    name: bimModelArtifactGuardString(row["name"], `${at}.name`),
    level: bimModelArtifactGuardInteger(row["level"], `${at}.level`, {"minimum": -2147483648, "maximum": 2147483647}),
    height: bimModelArtifactGuardNumber(row["height"], `${at}.height`),
    cut_height: row["cut_height"] === undefined ? undefined : bimModelArtifactGuardNumber(row["cut_height"], `${at}.cut_height`),
  };
}

export function parseStringerKind(value: unknown, at = "$"): StringerKind {
  return bimModelArtifactGuardMember(value, `${at}`, ["None", "Closed", "Open", "Mono"] as const);
}

export function parseStructuralLoad(value: unknown, at = "$"): StructuralLoad {
  const row = bimModelArtifactGuardClosed(bimModelArtifactGuardObject(value, at), at, ["name", "load_case", "member", "location", "force", "moment"]);
  return {
    name: bimModelArtifactGuardString(row["name"], `${at}.name`),
    load_case: bimModelArtifactGuardString(row["load_case"], `${at}.load_case`),
    member: bimModelArtifactGuardString(row["member"], `${at}.member`),
    location: parseStructuralLocation(row["location"], `${at}.location`),
    force: parsePoint3(row["force"], `${at}.force`),
    moment: parsePoint3(row["moment"], `${at}.moment`),
  };
}

export function parseStructuralLocation(value: unknown, at = "$"): StructuralLocation {
  return ((input0: unknown) => { if (typeof input0 === "object" && input0 !== null && !Array.isArray(input0)) { try { return ((row1) => ({ Point: parsePointLocation(row1["Point"], `${at}.Point`) }))(bimModelArtifactGuardClosed(bimModelArtifactGuardObject(input0, `${at}`), `${at}`, ["Point"])); } catch (error) { if (!(error instanceof bimModelArtifactGuardRefusal)) throw error; } } if (typeof input0 === "object" && input0 !== null && !Array.isArray(input0)) { try { return ((row1) => ({ Line: parseLineLocation(row1["Line"], `${at}.Line`) }))(bimModelArtifactGuardClosed(bimModelArtifactGuardObject(input0, `${at}`), `${at}`, ["Line"])); } catch (error) { if (!(error instanceof bimModelArtifactGuardRefusal)) throw error; } } if (typeof input0 === "string") { try { return bimModelArtifactGuardConstant(input0, `${at}`, "Area"); } catch (error) { if (!(error instanceof bimModelArtifactGuardRefusal)) throw error; } } return bimModelArtifactGuardReject(`${at}`, "value matches no variant"); })(value);
}

export function parseStructuralSupport(value: unknown, at = "$"): StructuralSupport {
  const row = bimModelArtifactGuardClosed(bimModelArtifactGuardObject(value, at), at, ["name", "member", "location", "offset", "restraints"]);
  return {
    name: bimModelArtifactGuardString(row["name"], `${at}.name`),
    member: bimModelArtifactGuardString(row["member"], `${at}.member`),
    location: parseStructuralLocation(row["location"], `${at}.location`),
    offset: parsePoint3(row["offset"], `${at}.offset`),
    restraints: parseRestraints(row["restraints"], `${at}.restraints`),
  };
}

export function parseSwing(value: unknown, at = "$"): Swing {
  return bimModelArtifactGuardMember(value, `${at}`, ["Left", "Right"] as const);
}

export function parseTag(value: unknown, at = "$"): Tag {
  const row = bimModelArtifactGuardClosed(bimModelArtifactGuardObject(value, at), at, ["storey", "element", "category", "offset", "style"]);
  return {
    storey: bimModelArtifactGuardString(row["storey"], `${at}.storey`),
    element: bimModelArtifactGuardString(row["element"], `${at}.element`),
    category: parseTagCategory(row["category"], `${at}.category`),
    offset: parsePoint2(row["offset"], `${at}.offset`),
    style: bimModelArtifactGuardString(row["style"], `${at}.style`),
  };
}

export function parseTagCategory(value: unknown, at = "$"): TagCategory {
  return bimModelArtifactGuardMember(value, `${at}`, ["Name", "Type", "Number", "Size"] as const);
}

export function parseTemplateTarget(value: unknown, at = "$"): TemplateTarget {
  return bimModelArtifactGuardMember(value, `${at}`, ["Site", "Building", "Storey", "Wall", "CurtainWall", "Column", "Beam", "Slab", "Ceiling", "Roof", "Window", "Door", "Void", "Stair", "Ramp", "Railing", "Space", "Zone", "WallType", "SlabType", "CeilingType", "RoofType", "ColumnType", "BeamType", "WindowType", "DoorType"] as const);
}

export function parseTerminator(value: unknown, at = "$"): Terminator {
  return bimModelArtifactGuardMember(value, `${at}`, ["Tick", "Arrow", "Dot"] as const);
}

export function parseTextNote(value: unknown, at = "$"): TextNote {
  const row = bimModelArtifactGuardClosed(bimModelArtifactGuardObject(value, at), at, ["storey", "position", "text", "rotation", "style"]);
  return {
    storey: bimModelArtifactGuardString(row["storey"], `${at}.storey`),
    position: parsePoint2(row["position"], `${at}.position`),
    text: bimModelArtifactGuardString(row["text"], `${at}.text`),
    rotation: bimModelArtifactGuardNumber(row["rotation"], `${at}.rotation`),
    style: bimModelArtifactGuardString(row["style"], `${at}.style`),
  };
}

export function parseTopConstraint(value: unknown, at = "$"): TopConstraint {
  return ((row0) => { if (row0["Unconnected"] !== undefined) { bimModelArtifactGuardClosed(row0, `${at}`, ["Unconnected"]); return { Unconnected: ((row1) => ({ height: bimModelArtifactGuardNumber(row1["height"], `${at}.Unconnected.height`) }))(bimModelArtifactGuardClosed(bimModelArtifactGuardObject(row0["Unconnected"], `${at}.Unconnected`), `${at}.Unconnected`, ["height"])) }; } if (row0["StoreyTop"] !== undefined) { bimModelArtifactGuardClosed(row0, `${at}`, ["StoreyTop"]); return { StoreyTop: ((row1) => ({ offset: bimModelArtifactGuardNumber(row1["offset"], `${at}.StoreyTop.offset`) }))(bimModelArtifactGuardClosed(bimModelArtifactGuardObject(row0["StoreyTop"], `${at}.StoreyTop`), `${at}.StoreyTop`, ["offset"])) }; } if (row0["Storey"] !== undefined) { bimModelArtifactGuardClosed(row0, `${at}`, ["Storey"]); return { Storey: ((row1) => ({ storey: bimModelArtifactGuardString(row1["storey"], `${at}.Storey.storey`), offset: bimModelArtifactGuardNumber(row1["offset"], `${at}.Storey.offset`) }))(bimModelArtifactGuardClosed(bimModelArtifactGuardObject(row0["Storey"], `${at}.Storey`), `${at}.Storey`, ["storey", "offset"])) }; } if (row0["Roof"] !== undefined) { bimModelArtifactGuardClosed(row0, `${at}`, ["Roof"]); return { Roof: ((row1) => ({ roof: bimModelArtifactGuardString(row1["roof"], `${at}.Roof.roof`), offset: bimModelArtifactGuardNumber(row1["offset"], `${at}.Roof.offset`) }))(bimModelArtifactGuardClosed(bimModelArtifactGuardObject(row0["Roof"], `${at}.Roof`), `${at}.Roof`, ["roof", "offset"])) }; } if (row0["Slab"] !== undefined) { bimModelArtifactGuardClosed(row0, `${at}`, ["Slab"]); return { Slab: ((row1) => ({ slab: bimModelArtifactGuardString(row1["slab"], `${at}.Slab.slab`), offset: bimModelArtifactGuardNumber(row1["offset"], `${at}.Slab.offset`) }))(bimModelArtifactGuardClosed(bimModelArtifactGuardObject(row0["Slab"], `${at}.Slab`), `${at}.Slab`, ["slab", "offset"])) }; } if (row0["Ceiling"] !== undefined) { bimModelArtifactGuardClosed(row0, `${at}`, ["Ceiling"]); return { Ceiling: ((row1) => ({ ceiling: bimModelArtifactGuardString(row1["ceiling"], `${at}.Ceiling.ceiling`), offset: bimModelArtifactGuardNumber(row1["offset"], `${at}.Ceiling.offset`) }))(bimModelArtifactGuardClosed(bimModelArtifactGuardObject(row0["Ceiling"], `${at}.Ceiling`), `${at}.Ceiling`, ["ceiling", "offset"])) }; } return bimModelArtifactGuardReject(`${at}`, "value matches no variant"); })(bimModelArtifactGuardObject(value, `${at}`));
}

export function parseTurn(value: unknown, at = "$"): Turn {
  return bimModelArtifactGuardMember(value, `${at}`, ["Left", "Right"] as const);
}

export function parseVertex(value: unknown, at = "$"): Vertex {
  const row = bimModelArtifactGuardClosed(bimModelArtifactGuardObject(value, at), at, ["point", "bulge"]);
  return {
    point: parsePoint2(row["point"], `${at}.point`),
    bulge: bimModelArtifactGuardNumber(row["bulge"], `${at}.bulge`),
  };
}

export function parseView(value: unknown, at = "$"): View {
  const row = bimModelArtifactGuardClosed(bimModelArtifactGuardObject(value, at), at, ["building", "name", "kind", "storey", "plane", "camera", "cut_height", "depth", "crop", "hidden", "phase", "scale", "detail"]);
  return {
    building: bimModelArtifactGuardString(row["building"], `${at}.building`),
    name: bimModelArtifactGuardString(row["name"], `${at}.name`),
    kind: parseViewKind(row["kind"], `${at}.kind`),
    storey: row["storey"] === undefined ? undefined : bimModelArtifactGuardString(row["storey"], `${at}.storey`),
    plane: row["plane"] === undefined ? undefined : parseViewPlane(row["plane"], `${at}.plane`),
    camera: row["camera"] === undefined ? undefined : parseViewCamera(row["camera"], `${at}.camera`),
    cut_height: row["cut_height"] === undefined ? undefined : bimModelArtifactGuardNumber(row["cut_height"], `${at}.cut_height`),
    depth: bimModelArtifactGuardNumber(row["depth"], `${at}.depth`),
    crop: row["crop"] === undefined ? undefined : parseViewCrop(row["crop"], `${at}.crop`),
    hidden: bimModelArtifactGuardArray(row["hidden"], `${at}.hidden`).map((item, index) => parseViewCategory(item, `${at}.hidden[${index}]`)),
    phase: row["phase"] === undefined ? undefined : parsePhase(row["phase"], `${at}.phase`),
    scale: bimModelArtifactGuardInteger(row["scale"], `${at}.scale`, {"minimum": 0, "maximum": 4294967295}),
    detail: parseDetailLevel(row["detail"], `${at}.detail`),
  };
}

export function parseViewCamera(value: unknown, at = "$"): ViewCamera {
  const row = bimModelArtifactGuardClosed(bimModelArtifactGuardObject(value, at), at, ["target", "target_height", "azimuth", "pitch", "distance"]);
  return {
    target: parsePoint2(row["target"], `${at}.target`),
    target_height: bimModelArtifactGuardNumber(row["target_height"], `${at}.target_height`),
    azimuth: bimModelArtifactGuardNumber(row["azimuth"], `${at}.azimuth`),
    pitch: bimModelArtifactGuardNumber(row["pitch"], `${at}.pitch`),
    distance: bimModelArtifactGuardNumber(row["distance"], `${at}.distance`),
  };
}

export function parseViewCategory(value: unknown, at = "$"): ViewCategory {
  return bimModelArtifactGuardMember(value, `${at}`, ["Walls", "CurtainWalls", "Columns", "Beams", "Slabs", "Roofs", "Openings", "Stairs", "Railings", "Spaces", "Grids"] as const);
}

export function parseViewCrop(value: unknown, at = "$"): ViewCrop {
  const row = bimModelArtifactGuardClosed(bimModelArtifactGuardObject(value, at), at, ["min", "max"]);
  return {
    min: parsePoint2(row["min"], `${at}.min`),
    max: parsePoint2(row["max"], `${at}.max`),
  };
}

export function parseViewKind(value: unknown, at = "$"): ViewKind {
  return bimModelArtifactGuardMember(value, `${at}`, ["Plan", "CeilingPlan", "Section", "Elevation", "Orthographic", "Perspective"] as const);
}

export function parseViewPlane(value: unknown, at = "$"): ViewPlane {
  const row = bimModelArtifactGuardClosed(bimModelArtifactGuardObject(value, at), at, ["start", "end"]);
  return {
    start: parsePoint2(row["start"], `${at}.start`),
    end: parsePoint2(row["end"], `${at}.end`),
  };
}

export function parseViewport(value: unknown, at = "$"): Viewport {
  const row = bimModelArtifactGuardClosed(bimModelArtifactGuardObject(value, at), at, ["sheet", "view", "position", "scale", "crop", "label"]);
  return {
    sheet: bimModelArtifactGuardString(row["sheet"], `${at}.sheet`),
    view: bimModelArtifactGuardString(row["view"], `${at}.view`),
    position: parsePoint2(row["position"], `${at}.position`),
    scale: bimModelArtifactGuardInteger(row["scale"], `${at}.scale`, {"minimum": 0, "maximum": 4294967295}),
    crop: row["crop"] === undefined ? undefined : parseViewCrop(row["crop"], `${at}.crop`),
    label: row["label"] === undefined ? undefined : bimModelArtifactGuardString(row["label"], `${at}.label`),
  };
}

export function parseWall(value: unknown, at = "$"): Wall {
  const row = bimModelArtifactGuardClosed(bimModelArtifactGuardObject(value, at), at, ["storey", "wall_type", "axis", "location", "base_offset", "top", "phase", "start_join", "end_join", "name", "base_slab"]);
  return {
    storey: bimModelArtifactGuardString(row["storey"], `${at}.storey`),
    wall_type: bimModelArtifactGuardString(row["wall_type"], `${at}.wall_type`),
    axis: parseAxis(row["axis"], `${at}.axis`),
    location: parseLocationLine(row["location"], `${at}.location`),
    base_offset: bimModelArtifactGuardNumber(row["base_offset"], `${at}.base_offset`),
    top: parseTopConstraint(row["top"], `${at}.top`),
    phase: parsePhase(row["phase"], `${at}.phase`),
    start_join: row["start_join"] === undefined ? undefined : parseEndJoin(row["start_join"], `${at}.start_join`),
    end_join: row["end_join"] === undefined ? undefined : parseEndJoin(row["end_join"], `${at}.end_join`),
    name: bimModelArtifactGuardString(row["name"], `${at}.name`),
    base_slab: row["base_slab"] === undefined ? undefined : bimModelArtifactGuardString(row["base_slab"], `${at}.base_slab`),
  };
}

export function parseWallSide(value: unknown, at = "$"): WallSide {
  return bimModelArtifactGuardMember(value, `${at}`, ["Left", "Right"] as const);
}

export function parseWallSweep(value: unknown, at = "$"): WallSweep {
  const row = bimModelArtifactGuardClosed(bimModelArtifactGuardObject(value, at), at, ["host", "side", "profile", "height", "inset", "material", "name"]);
  return {
    host: bimModelArtifactGuardString(row["host"], `${at}.host`),
    side: parseWallSide(row["side"], `${at}.side`),
    profile: parseProfile(row["profile"], `${at}.profile`),
    height: bimModelArtifactGuardNumber(row["height"], `${at}.height`),
    inset: bimModelArtifactGuardNumber(row["inset"], `${at}.inset`),
    material: bimModelArtifactGuardString(row["material"], `${at}.material`),
    name: bimModelArtifactGuardString(row["name"], `${at}.name`),
  };
}

export function parseWallType(value: unknown, at = "$"): WallType {
  const row = bimModelArtifactGuardClosed(bimModelArtifactGuardObject(value, at), at, ["name", "layers"]);
  return {
    name: bimModelArtifactGuardString(row["name"], `${at}.name`),
    layers: bimModelArtifactGuardArray(row["layers"], `${at}.layers`).map((item, index) => parseLayer(item, `${at}.layers[${index}]`)),
  };
}

export function parseWindowType(value: unknown, at = "$"): WindowType {
  const row = bimModelArtifactGuardClosed(bimModelArtifactGuardObject(value, at), at, ["name", "width", "height", "sill", "frame_width", "frame_depth", "panes", "material", "u_value", "g_value", "frame_fraction"]);
  return {
    name: bimModelArtifactGuardString(row["name"], `${at}.name`),
    width: bimModelArtifactGuardNumber(row["width"], `${at}.width`),
    height: bimModelArtifactGuardNumber(row["height"], `${at}.height`),
    sill: bimModelArtifactGuardNumber(row["sill"], `${at}.sill`),
    frame_width: bimModelArtifactGuardNumber(row["frame_width"], `${at}.frame_width`),
    frame_depth: bimModelArtifactGuardNumber(row["frame_depth"], `${at}.frame_depth`),
    panes: bimModelArtifactGuardInteger(row["panes"], `${at}.panes`, {"minimum": 0, "maximum": 4294967295}),
    material: bimModelArtifactGuardString(row["material"], `${at}.material`),
    u_value: row["u_value"] === undefined ? undefined : bimModelArtifactGuardNumber(row["u_value"], `${at}.u_value`),
    g_value: row["g_value"] === undefined ? undefined : bimModelArtifactGuardNumber(row["g_value"], `${at}.g_value`),
    frame_fraction: row["frame_fraction"] === undefined ? undefined : bimModelArtifactGuardNumber(row["frame_fraction"], `${at}.frame_fraction`),
  };
}

export function parseWorkset(value: unknown, at = "$"): Workset {
  const row = bimModelArtifactGuardClosed(bimModelArtifactGuardObject(value, at), at, ["name", "default_visible"]);
  return {
    name: bimModelArtifactGuardString(row["name"], `${at}.name`, {"minLength": 1}),
    default_visible: bimModelArtifactGuardBoolean(row["default_visible"], `${at}.default_visible`),
  };
}

export function parseZone(value: unknown, at = "$"): Zone {
  const row = bimModelArtifactGuardClosed(bimModelArtifactGuardObject(value, at), at, ["name", "category", "occupancy_density"]);
  return {
    name: bimModelArtifactGuardString(row["name"], `${at}.name`),
    category: bimModelArtifactGuardString(row["category"], `${at}.category`),
    occupancy_density: bimModelArtifactGuardNumber(row["occupancy_density"], `${at}.occupancy_density`),
  };
}
