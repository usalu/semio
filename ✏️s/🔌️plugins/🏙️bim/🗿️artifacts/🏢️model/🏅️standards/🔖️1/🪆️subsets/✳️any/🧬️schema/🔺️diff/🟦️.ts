/** 🔺️ BIM model diff schema — a sparse per-collection keyed delta; absent collections are untouched. */

import type { Point2, Rgb, Vertex, Slope, StairStringer, Baluster, Layer, Axis, TopConstraint, Profile, RoofShape, OpeningKind, StairFlight, PropertyValue, Infill, SpaceBoundary, MaterialCategory, LayerFunction, LocationLine, Phase, DoorLeaves, Swing, Turn, StringerKind, RiserKind, Material, WallType, SlabType, RoofType, ColumnType, BeamType, WindowType, DoorType, Site, Building, Storey, GridLine, Wall, CurtainWall, Column, Beam, Slab, Roof, Opening, Stair, Railing, Space, Classification, Project, PropertySet } from "../🟦️.ts";

export interface Assigned<T> {
  value: T;
}

export interface ProjectPatch {
  name?: string;
  description?: string;
  author?: string;
  organization?: string;
  phase_names?: string[];
}

export interface MaterialPatch {
  name?: string;
  category?: MaterialCategory;
  color?: Rgb;
  density?: number;
  conductivity?: number;
  specific_heat?: number;
}

export interface WallTypePatch {
  name?: string;
  layers?: Layer[];
}

export interface SlabTypePatch {
  name?: string;
  layers?: Layer[];
}

export interface RoofTypePatch {
  name?: string;
  layers?: Layer[];
}

export interface ColumnTypePatch {
  name?: string;
  profile?: Profile;
  material?: string;
}

export interface BeamTypePatch {
  name?: string;
  profile?: Profile;
  material?: string;
}

export interface WindowTypePatch {
  name?: string;
  width?: number;
  height?: number;
  sill?: number;
  frame_width?: number;
  frame_depth?: number;
  panes?: number;
  material?: string;
}

export interface DoorTypePatch {
  name?: string;
  width?: number;
  height?: number;
  frame_width?: number;
  frame_depth?: number;
  leaves?: DoorLeaves;
  swing?: Swing;
  material?: string;
}

export interface SitePatch {
  name?: string;
  latitude?: number;
  longitude?: number;
  elevation?: number;
  true_north?: number;
  boundary?: Point2[];
}

export interface BuildingPatch {
  site?: string;
  name?: string;
  origin?: Point2;
  rotation?: number;
  elevation?: number;
}

export interface StoreyPatch {
  building?: string;
  name?: string;
  level?: number;
  height?: number;
  cut_height?: Assigned<number | null>;
}

export interface GridLinePatch {
  building?: string;
  label?: string;
  start?: Point2;
  end?: Point2;
}

export interface WallPatch {
  storey?: string;
  wall_type?: string;
  axis?: Axis;
  location?: LocationLine;
  base_offset?: number;
  top?: TopConstraint;
  phase?: Phase;
  name?: string;
}

export interface CurtainWallPatch {
  storey?: string;
  axis?: Axis;
  base_offset?: number;
  top?: TopConstraint;
  u_spacing?: number;
  v_spacing?: number;
  mullion?: Profile;
  panel_material?: string;
  mullion_material?: string;
  name?: string;
}

export interface ColumnPatch {
  storey?: string;
  column_type?: string;
  position?: Point2;
  rotation?: number;
  base_offset?: number;
  top?: TopConstraint;
  name?: string;
}

export interface BeamPatch {
  storey?: string;
  beam_type?: string;
  start?: Point2;
  end?: Point2;
  top_offset?: number;
  name?: string;
}

export interface SlabPatch {
  storey?: string;
  slab_type?: string;
  boundary?: Vertex[];
  holes?: Vertex[][];
  offset?: number;
  slope?: Assigned<Slope | null>;
  name?: string;
}

export interface RoofPatch {
  storey?: string;
  roof_type?: string;
  footprint?: Vertex[];
  shape?: RoofShape;
  overhang?: number;
  base_offset?: number;
  name?: string;
}

export interface OpeningPatch {
  host?: string;
  kind?: OpeningKind;
  offset?: number;
  sill_override?: Assigned<number | null>;
  width?: Assigned<number | null>;
  height?: Assigned<number | null>;
  flip_hand?: boolean;
  flip_facing?: boolean;
  name?: string;
}

export interface StairPatch {
  storey?: string;
  start?: Point2;
  direction?: number;
  width?: number;
  flight?: StairFlight;
  top?: TopConstraint;
  max_riser?: number;
  min_tread?: number;
  stringer?: StairStringer;
  nosing?: number;
  tread_thickness?: number;
  riser?: RiserKind;
  landing_depth?: number;
  name?: string;
}

export interface RailingPatch {
  storey?: string;
  path?: Point2[];
  height?: number;
  post_spacing?: number;
  profile?: Profile;
  post_profile?: Profile;
  baluster?: Assigned<Baluster | null>;
  infill?: Infill;
  material?: string;
  base_offset?: number;
  name?: string;
}

export interface SpacePatch {
  storey?: string;
  number?: string;
  name?: string;
  boundary?: SpaceBoundary;
  usage?: string;
}

export interface ClassificationPatch {
  system?: string;
  code?: string;
  title?: string;
}

export interface PropertySetPatch {
  assigned?: Record<string, Record<string, PropertyValue | null>>;
}

export type MaterialEntry = ({ entry: "Created" } & Material) | { entry: "Deleted" } | ({ entry: "Replaced" } & Material) | ({ entry: "Patched" } & MaterialPatch);
export type WallTypeEntry = ({ entry: "Created" } & WallType) | { entry: "Deleted" } | ({ entry: "Replaced" } & WallType) | ({ entry: "Patched" } & WallTypePatch);
export type SlabTypeEntry = ({ entry: "Created" } & SlabType) | { entry: "Deleted" } | ({ entry: "Replaced" } & SlabType) | ({ entry: "Patched" } & SlabTypePatch);
export type RoofTypeEntry = ({ entry: "Created" } & RoofType) | { entry: "Deleted" } | ({ entry: "Replaced" } & RoofType) | ({ entry: "Patched" } & RoofTypePatch);
export type ColumnTypeEntry = ({ entry: "Created" } & ColumnType) | { entry: "Deleted" } | ({ entry: "Replaced" } & ColumnType) | ({ entry: "Patched" } & ColumnTypePatch);
export type BeamTypeEntry = ({ entry: "Created" } & BeamType) | { entry: "Deleted" } | ({ entry: "Replaced" } & BeamType) | ({ entry: "Patched" } & BeamTypePatch);
export type WindowTypeEntry = ({ entry: "Created" } & WindowType) | { entry: "Deleted" } | ({ entry: "Replaced" } & WindowType) | ({ entry: "Patched" } & WindowTypePatch);
export type DoorTypeEntry = ({ entry: "Created" } & DoorType) | { entry: "Deleted" } | ({ entry: "Replaced" } & DoorType) | ({ entry: "Patched" } & DoorTypePatch);
export type SiteEntry = ({ entry: "Created" } & Site) | { entry: "Deleted" } | ({ entry: "Replaced" } & Site) | ({ entry: "Patched" } & SitePatch);
export type BuildingEntry = ({ entry: "Created" } & Building) | { entry: "Deleted" } | ({ entry: "Replaced" } & Building) | ({ entry: "Patched" } & BuildingPatch);
export type StoreyEntry = ({ entry: "Created" } & Storey) | { entry: "Deleted" } | ({ entry: "Replaced" } & Storey) | ({ entry: "Patched" } & StoreyPatch);
export type GridLineEntry = ({ entry: "Created" } & GridLine) | { entry: "Deleted" } | ({ entry: "Replaced" } & GridLine) | ({ entry: "Patched" } & GridLinePatch);
export type WallEntry = ({ entry: "Created" } & Wall) | { entry: "Deleted" } | ({ entry: "Replaced" } & Wall) | ({ entry: "Patched" } & WallPatch);
export type CurtainWallEntry = ({ entry: "Created" } & CurtainWall) | { entry: "Deleted" } | ({ entry: "Replaced" } & CurtainWall) | ({ entry: "Patched" } & CurtainWallPatch);
export type ColumnEntry = ({ entry: "Created" } & Column) | { entry: "Deleted" } | ({ entry: "Replaced" } & Column) | ({ entry: "Patched" } & ColumnPatch);
export type BeamEntry = ({ entry: "Created" } & Beam) | { entry: "Deleted" } | ({ entry: "Replaced" } & Beam) | ({ entry: "Patched" } & BeamPatch);
export type SlabEntry = ({ entry: "Created" } & Slab) | { entry: "Deleted" } | ({ entry: "Replaced" } & Slab) | ({ entry: "Patched" } & SlabPatch);
export type RoofEntry = ({ entry: "Created" } & Roof) | { entry: "Deleted" } | ({ entry: "Replaced" } & Roof) | ({ entry: "Patched" } & RoofPatch);
export type OpeningEntry = ({ entry: "Created" } & Opening) | { entry: "Deleted" } | ({ entry: "Replaced" } & Opening) | ({ entry: "Patched" } & OpeningPatch);
export type StairEntry = ({ entry: "Created" } & Stair) | { entry: "Deleted" } | ({ entry: "Replaced" } & Stair) | ({ entry: "Patched" } & StairPatch);
export type RailingEntry = ({ entry: "Created" } & Railing) | { entry: "Deleted" } | ({ entry: "Replaced" } & Railing) | ({ entry: "Patched" } & RailingPatch);
export type SpaceEntry = ({ entry: "Created" } & Space) | { entry: "Deleted" } | ({ entry: "Replaced" } & Space) | ({ entry: "Patched" } & SpacePatch);
export type PropertySetEntry = ({ entry: "Created" } & PropertySet) | { entry: "Deleted" } | ({ entry: "Replaced" } & PropertySet) | ({ entry: "Patched" } & PropertySetPatch);
export type ClassificationEntry = ({ entry: "Created" } & Classification) | { entry: "Deleted" } | ({ entry: "Replaced" } & Classification) | ({ entry: "Patched" } & ClassificationPatch);

export interface ModelDiff {
  project?: ProjectPatch;
  materials?: Record<string, MaterialEntry>;
  wall_types?: Record<string, WallTypeEntry>;
  slab_types?: Record<string, SlabTypeEntry>;
  roof_types?: Record<string, RoofTypeEntry>;
  column_types?: Record<string, ColumnTypeEntry>;
  beam_types?: Record<string, BeamTypeEntry>;
  window_types?: Record<string, WindowTypeEntry>;
  door_types?: Record<string, DoorTypeEntry>;
  sites?: Record<string, SiteEntry>;
  buildings?: Record<string, BuildingEntry>;
  storeys?: Record<string, StoreyEntry>;
  grids?: Record<string, GridLineEntry>;
  walls?: Record<string, WallEntry>;
  curtain_walls?: Record<string, CurtainWallEntry>;
  columns?: Record<string, ColumnEntry>;
  beams?: Record<string, BeamEntry>;
  slabs?: Record<string, SlabEntry>;
  roofs?: Record<string, RoofEntry>;
  openings?: Record<string, OpeningEntry>;
  stairs?: Record<string, StairEntry>;
  railings?: Record<string, RailingEntry>;
  spaces?: Record<string, SpaceEntry>;
  properties?: Record<string, PropertySetEntry>;
  classifications?: Record<string, ClassificationEntry>;
}
