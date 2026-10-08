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
  spaces?: Record<string, Space>;
  /** @state artifact */
  properties?: Record<string, PropertySet>;
  /** @state artifact */
  classifications?: Record<string, Classification>;
}

export type MaterialCategory = "Concrete" | "Masonry" | "Wood" | "Metal" | "Glass" | "Insulation" | "Finish" | "Membrane" | "Other";

export type LayerFunction = "Structure" | "Substrate" | "Insulation" | "Finish" | "Membrane" | "Core";

export type LocationLine = "Center" | "Interior" | "Exterior" | "CoreCenter";

export type Phase = "Existing" | "New" | "Demolished" | "Temporary";

export type DoorLeaves = "Single" | "Double";

export type Swing = "Left" | "Right";

export type Turn = "Left" | "Right";

export type StringerKind = "None" | "Closed" | "Open" | "Mono";

export type RiserKind = "Open" | "Closed";

export type Axis =
  | { Line: { start: Point2; end: Point2 } }
  | { Arc: { start: Point2; end: Point2; bulge: number } };

export type TopConstraint =
  | { Unconnected: { height: number } }
  | { StoreyTop: { offset: number } }
  | { Storey: { storey: string; offset: number } };

export type Profile =
  | { Rectangle: { width: number; depth: number } }
  | { Circle: { diameter: number } }
  | { IShape: { width: number; depth: number; web: number; flange: number } }
  | { Custom: { outline: Vertex[] } };

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
  cut_height?: number | null;
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
  name: string;
}

export interface CurtainWall {
  storey: string;
  axis: Axis;
  base_offset: number;
  top: TopConstraint;
  u_spacing: number;
  v_spacing: number;
  mullion: Profile;
  panel_material: string;
  mullion_material: string;
  name: string;
}

export interface Column {
  storey: string;
  column_type: string;
  position: Point2;
  rotation: number;
  base_offset: number;
  top: TopConstraint;
  name: string;
}

export interface Beam {
  storey: string;
  beam_type: string;
  start: Point2;
  end: Point2;
  top_offset: number;
  name: string;
}

export interface Slab {
  storey: string;
  slab_type: string;
  boundary: Vertex[];
  holes: Vertex[][];
  offset: number;
  slope?: Slope | null;
  name: string;
}

export interface Roof {
  storey: string;
  roof_type: string;
  footprint: Vertex[];
  shape: RoofShape;
  overhang: number;
  base_offset: number;
  name: string;
}

export interface Opening {
  host: string;
  kind: OpeningKind;
  offset: number;
  sill_override?: number | null;
  width?: number | null;
  height?: number | null;
  flip_hand: boolean;
  flip_facing: boolean;
  name: string;
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
  name: string;
}

export interface Railing {
  storey: string;
  path: Point2[];
  height: number;
  post_spacing: number;
  profile: Profile;
  post_profile: Profile;
  baluster?: Baluster | null;
  infill: Infill;
  material: string;
  base_offset: number;
  name: string;
}

export interface Space {
  storey: string;
  number: string;
  name: string;
  boundary: SpaceBoundary;
  usage: string;
}

export interface Classification {
  system: string;
  code: string;
  title: string;
}

export type PropertySet = Record<string, Record<string, PropertyValue>>;

