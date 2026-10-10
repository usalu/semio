/** 🔺️ BIM model diff schema — a sparse per-collection keyed delta; absent collections are untouched. */

import type { PropertyDef, ClassificationItem, ExprPoint, ExprPoint3, Point3, ElementSelector, RuleScope, ClashRef, SectionBox, IssueViewpoint, Point2, Rgb, Vertex, Slope, StairStringer, Baluster, RailingHost, Layer, ViewPlane, ViewCrop, ViewCamera, ScheduleColumn, ScheduleSort, ScheduleFilter, ScheduleGroup, AnnotationAnchor, ParametricProfile, SolidShape, MepShape, Paper, CurtainGrid, CurtainPanel, Axis, TopConstraint, Profile, RoofShape, OpeningKind, StairFlight, PropertyValue, Infill, SpaceBoundary, ScheduleKey, PropertyKind, TemplateTarget, AnchorEnd, WallSide, TagCategory, Terminator, DimensionUnit, FamilyCategory, ParameterKind, SolidAxis, MepSystem, ElementClass, RuleKind, RuleSeverity, IssueStatus, IssuePriority, IsoSize, Orientation, MaterialCategory, LayerFunction, LocationLine, Phase, DoorLeaves, Swing, Turn, StringerKind, RiserKind, ScheduleCategory, ScheduleField, ScheduleOp, EndJoin, ViewKind, DetailLevel, ViewCategory, HostSide, AreaMeasure, PropertyTemplate, ClassificationSystem, SpaceConditions, Dimension, Tag, TextNote, Leader, AnnotationStyle, Family, FamilyParameter, FamilySolid, Component, ComponentOverride, MepElement, ClashSet, Rule, Issue, IssueComment, Sheet, Viewport, SheetRevision, WallSweep, Material, WallType, SlabType, CeilingType, RoofType, ColumnType, BeamType, WindowType, DoorType, Site, Building, Storey, GridLine, Wall, CurtainWallType, CurtainPanelOverride, CurtainWall, Column, Beam, Slab, Ceiling, Roof, Opening, Stair, Railing, Ramp, Space, Zone, AreaScheme, View, Schedule, Project, PropertySet, ClassificationSet } from "../🟦️.ts";

export interface Assigned<T> {
  value: T;
}

export interface PropertyTemplatePatch {
  name?: string;
  applies_to?: TemplateTarget[];
  properties?: PropertyDef[];
}

export interface ClassificationSystemPatch {
  name?: string;
  edition?: string;
  source?: Assigned<string | null>;
  entries?: ClassificationItem[];
}

export interface SpaceConditionsPatch {
  occupancy?: Assigned<string | null>;
  occupancy_density?: Assigned<number | null>;
  heating_setpoint?: Assigned<number | null>;
  cooling_setpoint?: Assigned<number | null>;
  ventilation_rate?: Assigned<number | null>;
  lighting_power_density?: Assigned<number | null>;
  equipment_power_density?: Assigned<number | null>;
  schedule?: Assigned<string | null>;
}

export interface DimensionPatch {
  storey?: string;
  anchors?: AnnotationAnchor[];
  angle?: number;
  offset?: number;
  style?: string;
  lock?: Assigned<number | null>;
  name?: string;
}

export interface TagPatch {
  storey?: string;
  element?: string;
  category?: TagCategory;
  offset?: Point2;
  style?: string;
}

export interface TextNotePatch {
  storey?: string;
  position?: Point2;
  text?: string;
  rotation?: number;
  style?: string;
}

export interface LeaderPatch {
  storey?: string;
  anchor?: AnnotationAnchor;
  offset?: Point2;
  text?: string;
  style?: string;
}

export interface AnnotationStylePatch {
  name?: string;
  text_height?: number;
  terminator?: Terminator;
  unit?: DimensionUnit;
  precision?: number;
  mark_size?: number;
  gap?: number;
  overshoot?: number;
}

export interface FamilyPatch {
  name?: string;
  category?: FamilyCategory;
}

export interface FamilyParameterPatch {
  family?: string;
  name?: string;
  kind?: ParameterKind;
  value?: string;
}

export interface FamilySolidPatch {
  family?: string;
  name?: string;
  shape?: SolidShape;
  material?: string;
  visible?: string;
  offset?: ExprPoint3;
}

export interface ComponentPatch {
  storey?: string;
  family?: string;
  position?: Point2;
  elevation?: number;
  rotation?: number;
  mirrored?: boolean;
  host?: Assigned<string | null>;
  system?: Assigned<MepSystem | null>;
  name?: string;
}

export interface ComponentOverridePatch {
  component?: string;
  name?: string;
  value?: string;
}

export interface MepElementPatch {
  storey?: string;
  system?: MepSystem;
  shape?: MepShape;
  path?: Point3[];
  name?: string;
}

export interface ClashSetPatch {
  name?: string;
  a?: ElementSelector;
  b?: ElementSelector;
  tolerance?: number;
  clearance?: number;
}

export interface RulePatch {
  name?: string;
  kind?: RuleKind;
  limit?: number;
  severity?: RuleSeverity;
  scope?: RuleScope;
}

export interface IssuePatch {
  title?: string;
  description?: string;
  status?: IssueStatus;
  priority?: IssuePriority;
  assignee?: string;
  author?: string;
  created?: string;
  labels?: string[];
  elements?: string[];
  clash?: Assigned<ClashRef | null>;
  viewpoint?: Assigned<IssueViewpoint | null>;
}

export interface IssueCommentPatch {
  issue?: string;
  author?: string;
  date?: string;
  text?: string;
}

export interface SheetPatch {
  number?: string;
  name?: string;
  paper?: Paper;
  orientation?: Orientation;
  project?: string;
  drawn_by?: string;
  checked_by?: string;
  date?: string;
  revision?: string;
  scale_label?: string;
}

export interface ViewportPatch {
  sheet?: string;
  view?: string;
  position?: Point2;
  scale?: number;
  crop?: Assigned<ViewCrop | null>;
  label?: Assigned<string | null>;
}

export interface SheetRevisionPatch {
  sheet?: string;
  number?: string;
  date?: string;
  description?: string;
  author?: string;
}

export interface WallSweepPatch {
  host?: string;
  side?: WallSide;
  profile?: Profile;
  height?: number;
  inset?: number;
  material?: string;
  name?: string;
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

export interface CeilingTypePatch {
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
  u_value?: Assigned<number | null>;
  g_value?: Assigned<number | null>;
  frame_fraction?: Assigned<number | null>;
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
  u_value?: Assigned<number | null>;
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
  start_join?: Assigned<EndJoin | null>;
  end_join?: Assigned<EndJoin | null>;
  name?: string;
  base_slab?: Assigned<string | null>;
}

export interface CurtainWallTypePatch {
  name?: string;
  u_grid?: CurtainGrid;
  v_grid?: CurtainGrid;
  interior_mullion?: Profile;
  border_mullion?: Profile;
  panel?: CurtainPanel;
  panel_material?: string;
  mullion_material?: string;
  u_value?: Assigned<number | null>;
  g_value?: Assigned<number | null>;
  frame_fraction?: Assigned<number | null>;
}

export interface CurtainPanelOverridePatch {
  curtain?: string;
  u?: number;
  v?: number;
  panel?: CurtainPanel;
}

export interface CurtainWallPatch {
  storey?: string;
  curtain_wall_type?: string;
  axis?: Axis;
  base_offset?: number;
  top?: TopConstraint;
  u_grid?: Assigned<CurtainGrid | null>;
  v_grid?: Assigned<CurtainGrid | null>;
  phase?: Phase;
  name?: string;
}

export interface ColumnPatch {
  storey?: string;
  column_type?: string;
  position?: Point2;
  rotation?: number;
  tilt?: Assigned<Slope | null>;
  base_offset?: number;
  top?: TopConstraint;
  phase?: Phase;
  name?: string;
}

export interface BeamPatch {
  storey?: string;
  beam_type?: string;
  axis?: Axis;
  top_offset?: number;
  end_top_offset?: Assigned<number | null>;
  phase?: Phase;
  name?: string;
}

export interface SlabPatch {
  storey?: string;
  slab_type?: string;
  boundary?: Vertex[];
  holes?: Vertex[][];
  offset?: number;
  slope?: Assigned<Slope | null>;
  phase?: Phase;
  name?: string;
}

export interface CeilingPatch {
  storey?: string;
  ceiling_type?: string;
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
  phase?: Phase;
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
  reveal_depth?: Assigned<number | null>;
  reveal_material?: Assigned<string | null>;
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
  phase?: Phase;
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
  host?: Assigned<RailingHost | null>;
  phase?: Phase;
  name?: string;
}

export interface RampPatch {
  storey?: string;
  path?: Vertex[];
  width?: number;
  landing_start?: number;
  landing_end?: number;
  landing_turn?: number;
  max_slope?: number;
  thickness?: number;
  material?: string;
  base_offset?: number;
  top?: TopConstraint;
  railing_left?: boolean;
  railing_right?: boolean;
  name?: string;
}

export interface SpacePatch {
  storey?: string;
  number?: string;
  name?: string;
  boundary?: SpaceBoundary;
  usage?: string;
  phase?: Phase;
  zone?: Assigned<string | null>;
  floor_finish?: Assigned<string | null>;
  wall_finish?: Assigned<string | null>;
  ceiling_finish?: Assigned<string | null>;
}

export interface ZonePatch {
  name?: string;
  category?: string;
  occupancy_density?: number;
}

export interface AreaSchemePatch {
  name?: string;
  measure?: AreaMeasure;
  usages?: string[];
  zones?: string[];
}

export interface ViewPatch {
  building?: string;
  name?: string;
  kind?: ViewKind;
  storey?: Assigned<string | null>;
  plane?: Assigned<ViewPlane | null>;
  camera?: Assigned<ViewCamera | null>;
  cut_height?: Assigned<number | null>;
  depth?: number;
  crop?: Assigned<ViewCrop | null>;
  hidden?: ViewCategory[];
  phase?: Assigned<Phase | null>;
  scale?: number;
  detail?: DetailLevel;
}

export interface SchedulePatch {
  name?: string;
  category?: ScheduleCategory;
  columns?: ScheduleColumn[];
  sort?: ScheduleSort[];
  filter?: ScheduleFilter[];
  group?: ScheduleGroup[];
  itemize?: boolean;
  storeys?: string[];
  phases?: Phase[];
}

export interface PropertySetPatch {
  assigned?: Record<string, Record<string, PropertyValue | null>>;
}

export interface ClassificationSetPatch {
  assigned?: Record<string, string | null>;
}

export type MaterialEntry = ({ entry: "Created" } & Material) | { entry: "Deleted" } | ({ entry: "Replaced" } & Material) | ({ entry: "Patched" } & MaterialPatch);
export type WallTypeEntry = ({ entry: "Created" } & WallType) | { entry: "Deleted" } | ({ entry: "Replaced" } & WallType) | ({ entry: "Patched" } & WallTypePatch);
export type SlabTypeEntry = ({ entry: "Created" } & SlabType) | { entry: "Deleted" } | ({ entry: "Replaced" } & SlabType) | ({ entry: "Patched" } & SlabTypePatch);
export type RoofTypeEntry = ({ entry: "Created" } & RoofType) | { entry: "Deleted" } | ({ entry: "Replaced" } & RoofType) | ({ entry: "Patched" } & RoofTypePatch);
export type ColumnTypeEntry = ({ entry: "Created" } & ColumnType) | { entry: "Deleted" } | ({ entry: "Replaced" } & ColumnType) | ({ entry: "Patched" } & ColumnTypePatch);
export type BeamTypeEntry = ({ entry: "Created" } & BeamType) | { entry: "Deleted" } | ({ entry: "Replaced" } & BeamType) | ({ entry: "Patched" } & BeamTypePatch);
export type WindowTypeEntry = ({ entry: "Created" } & WindowType) | { entry: "Deleted" } | ({ entry: "Replaced" } & WindowType) | ({ entry: "Patched" } & WindowTypePatch);
export type DoorTypeEntry = ({ entry: "Created" } & DoorType) | { entry: "Deleted" } | ({ entry: "Replaced" } & DoorType) | ({ entry: "Patched" } & DoorTypePatch);
export type CurtainWallTypeEntry = ({ entry: "Created" } & CurtainWallType) | { entry: "Deleted" } | ({ entry: "Replaced" } & CurtainWallType) | ({ entry: "Patched" } & CurtainWallTypePatch);
export type SiteEntry = ({ entry: "Created" } & Site) | { entry: "Deleted" } | ({ entry: "Replaced" } & Site) | ({ entry: "Patched" } & SitePatch);
export type BuildingEntry = ({ entry: "Created" } & Building) | { entry: "Deleted" } | ({ entry: "Replaced" } & Building) | ({ entry: "Patched" } & BuildingPatch);
export type StoreyEntry = ({ entry: "Created" } & Storey) | { entry: "Deleted" } | ({ entry: "Replaced" } & Storey) | ({ entry: "Patched" } & StoreyPatch);
export type GridLineEntry = ({ entry: "Created" } & GridLine) | { entry: "Deleted" } | ({ entry: "Replaced" } & GridLine) | ({ entry: "Patched" } & GridLinePatch);
export type WallEntry = ({ entry: "Created" } & Wall) | { entry: "Deleted" } | ({ entry: "Replaced" } & Wall) | ({ entry: "Patched" } & WallPatch);
export type CurtainWallEntry = ({ entry: "Created" } & CurtainWall) | { entry: "Deleted" } | ({ entry: "Replaced" } & CurtainWall) | ({ entry: "Patched" } & CurtainWallPatch);
export type CurtainPanelOverrideEntry = ({ entry: "Created" } & CurtainPanelOverride) | { entry: "Deleted" } | ({ entry: "Replaced" } & CurtainPanelOverride) | ({ entry: "Patched" } & CurtainPanelOverridePatch);
export type ColumnEntry = ({ entry: "Created" } & Column) | { entry: "Deleted" } | ({ entry: "Replaced" } & Column) | ({ entry: "Patched" } & ColumnPatch);
export type BeamEntry = ({ entry: "Created" } & Beam) | { entry: "Deleted" } | ({ entry: "Replaced" } & Beam) | ({ entry: "Patched" } & BeamPatch);
export type SlabEntry = ({ entry: "Created" } & Slab) | { entry: "Deleted" } | ({ entry: "Replaced" } & Slab) | ({ entry: "Patched" } & SlabPatch);
export type RoofEntry = ({ entry: "Created" } & Roof) | { entry: "Deleted" } | ({ entry: "Replaced" } & Roof) | ({ entry: "Patched" } & RoofPatch);
export type OpeningEntry = ({ entry: "Created" } & Opening) | { entry: "Deleted" } | ({ entry: "Replaced" } & Opening) | ({ entry: "Patched" } & OpeningPatch);
export type StairEntry = ({ entry: "Created" } & Stair) | { entry: "Deleted" } | ({ entry: "Replaced" } & Stair) | ({ entry: "Patched" } & StairPatch);
export type RailingEntry = ({ entry: "Created" } & Railing) | { entry: "Deleted" } | ({ entry: "Replaced" } & Railing) | ({ entry: "Patched" } & RailingPatch);
export type RampEntry = ({ entry: "Created" } & Ramp) | { entry: "Deleted" } | ({ entry: "Replaced" } & Ramp) | ({ entry: "Patched" } & RampPatch);
export type SpaceEntry = ({ entry: "Created" } & Space) | { entry: "Deleted" } | ({ entry: "Replaced" } & Space) | ({ entry: "Patched" } & SpacePatch);
export type CeilingTypeEntry = ({ entry: "Created" } & CeilingType) | { entry: "Deleted" } | ({ entry: "Replaced" } & CeilingType) | ({ entry: "Patched" } & CeilingTypePatch);
export type CeilingEntry = ({ entry: "Created" } & Ceiling) | { entry: "Deleted" } | ({ entry: "Replaced" } & Ceiling) | ({ entry: "Patched" } & CeilingPatch);
export type ZoneEntry = ({ entry: "Created" } & Zone) | { entry: "Deleted" } | ({ entry: "Replaced" } & Zone) | ({ entry: "Patched" } & ZonePatch);
export type AreaSchemeEntry = ({ entry: "Created" } & AreaScheme) | { entry: "Deleted" } | ({ entry: "Replaced" } & AreaScheme) | ({ entry: "Patched" } & AreaSchemePatch);
export type SpaceConditionsEntry = ({ entry: "Created" } & SpaceConditions) | { entry: "Deleted" } | ({ entry: "Replaced" } & SpaceConditions) | ({ entry: "Patched" } & SpaceConditionsPatch);
export type ViewEntry = ({ entry: "Created" } & View) | { entry: "Deleted" } | ({ entry: "Replaced" } & View) | ({ entry: "Patched" } & ViewPatch);
export type SheetEntry = ({ entry: "Created" } & Sheet) | { entry: "Deleted" } | ({ entry: "Replaced" } & Sheet) | ({ entry: "Patched" } & SheetPatch);
export type ViewportEntry = ({ entry: "Created" } & Viewport) | { entry: "Deleted" } | ({ entry: "Replaced" } & Viewport) | ({ entry: "Patched" } & ViewportPatch);
export type SheetRevisionEntry = ({ entry: "Created" } & SheetRevision) | { entry: "Deleted" } | ({ entry: "Replaced" } & SheetRevision) | ({ entry: "Patched" } & SheetRevisionPatch);
export type DimensionEntry = ({ entry: "Created" } & Dimension) | { entry: "Deleted" } | ({ entry: "Replaced" } & Dimension) | ({ entry: "Patched" } & DimensionPatch);
export type TagEntry = ({ entry: "Created" } & Tag) | { entry: "Deleted" } | ({ entry: "Replaced" } & Tag) | ({ entry: "Patched" } & TagPatch);
export type TextNoteEntry = ({ entry: "Created" } & TextNote) | { entry: "Deleted" } | ({ entry: "Replaced" } & TextNote) | ({ entry: "Patched" } & TextNotePatch);
export type LeaderEntry = ({ entry: "Created" } & Leader) | { entry: "Deleted" } | ({ entry: "Replaced" } & Leader) | ({ entry: "Patched" } & LeaderPatch);
export type AnnotationStyleEntry = ({ entry: "Created" } & AnnotationStyle) | { entry: "Deleted" } | ({ entry: "Replaced" } & AnnotationStyle) | ({ entry: "Patched" } & AnnotationStylePatch);
export type FamilyEntry = ({ entry: "Created" } & Family) | { entry: "Deleted" } | ({ entry: "Replaced" } & Family) | ({ entry: "Patched" } & FamilyPatch);
export type FamilyParameterEntry = ({ entry: "Created" } & FamilyParameter) | { entry: "Deleted" } | ({ entry: "Replaced" } & FamilyParameter) | ({ entry: "Patched" } & FamilyParameterPatch);
export type FamilySolidEntry = ({ entry: "Created" } & FamilySolid) | { entry: "Deleted" } | ({ entry: "Replaced" } & FamilySolid) | ({ entry: "Patched" } & FamilySolidPatch);
export type ComponentEntry = ({ entry: "Created" } & Component) | { entry: "Deleted" } | ({ entry: "Replaced" } & Component) | ({ entry: "Patched" } & ComponentPatch);
export type ComponentOverrideEntry = ({ entry: "Created" } & ComponentOverride) | { entry: "Deleted" } | ({ entry: "Replaced" } & ComponentOverride) | ({ entry: "Patched" } & ComponentOverridePatch);
export type MepElementEntry = ({ entry: "Created" } & MepElement) | { entry: "Deleted" } | ({ entry: "Replaced" } & MepElement) | ({ entry: "Patched" } & MepElementPatch);
export type ClashSetEntry = ({ entry: "Created" } & ClashSet) | { entry: "Deleted" } | ({ entry: "Replaced" } & ClashSet) | ({ entry: "Patched" } & ClashSetPatch);
export type RuleEntry = ({ entry: "Created" } & Rule) | { entry: "Deleted" } | ({ entry: "Replaced" } & Rule) | ({ entry: "Patched" } & RulePatch);
export type IssueEntry = ({ entry: "Created" } & Issue) | { entry: "Deleted" } | ({ entry: "Replaced" } & Issue) | ({ entry: "Patched" } & IssuePatch);
export type IssueCommentEntry = ({ entry: "Created" } & IssueComment) | { entry: "Deleted" } | ({ entry: "Replaced" } & IssueComment) | ({ entry: "Patched" } & IssueCommentPatch);
export type WallSweepEntry = ({ entry: "Created" } & WallSweep) | { entry: "Deleted" } | ({ entry: "Replaced" } & WallSweep) | ({ entry: "Patched" } & WallSweepPatch);
export type ScheduleEntry = ({ entry: "Created" } & Schedule) | { entry: "Deleted" } | ({ entry: "Replaced" } & Schedule) | ({ entry: "Patched" } & SchedulePatch);
export type PropertyTemplateEntry = ({ entry: "Created" } & PropertyTemplate) | { entry: "Deleted" } | ({ entry: "Replaced" } & PropertyTemplate) | ({ entry: "Patched" } & PropertyTemplatePatch);
export type ClassificationSystemEntry = ({ entry: "Created" } & ClassificationSystem) | { entry: "Deleted" } | ({ entry: "Replaced" } & ClassificationSystem) | ({ entry: "Patched" } & ClassificationSystemPatch);
export type PropertySetEntry = ({ entry: "Created" } & PropertySet) | { entry: "Deleted" } | ({ entry: "Replaced" } & PropertySet) | ({ entry: "Patched" } & PropertySetPatch);
export type ClassificationSetEntry = ({ entry: "Created" } & ClassificationSet) | { entry: "Deleted" } | ({ entry: "Replaced" } & ClassificationSet) | ({ entry: "Patched" } & ClassificationSetPatch);

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
  curtain_wall_types?: Record<string, CurtainWallTypeEntry>;
  sites?: Record<string, SiteEntry>;
  buildings?: Record<string, BuildingEntry>;
  storeys?: Record<string, StoreyEntry>;
  grids?: Record<string, GridLineEntry>;
  walls?: Record<string, WallEntry>;
  curtain_walls?: Record<string, CurtainWallEntry>;
  curtain_panel_overrides?: Record<string, CurtainPanelOverrideEntry>;
  columns?: Record<string, ColumnEntry>;
  beams?: Record<string, BeamEntry>;
  slabs?: Record<string, SlabEntry>;
  roofs?: Record<string, RoofEntry>;
  openings?: Record<string, OpeningEntry>;
  stairs?: Record<string, StairEntry>;
  railings?: Record<string, RailingEntry>;
  ramps?: Record<string, RampEntry>;
  spaces?: Record<string, SpaceEntry>;
  ceiling_types?: Record<string, CeilingTypeEntry>;
  ceilings?: Record<string, CeilingEntry>;
  zones?: Record<string, ZoneEntry>;
  area_schemes?: Record<string, AreaSchemeEntry>;
  space_conditions?: Record<string, SpaceConditionsEntry>;
  views?: Record<string, ViewEntry>;
  sheets?: Record<string, SheetEntry>;
  viewports?: Record<string, ViewportEntry>;
  sheet_revisions?: Record<string, SheetRevisionEntry>;
  dimensions?: Record<string, DimensionEntry>;
  tags?: Record<string, TagEntry>;
  text_notes?: Record<string, TextNoteEntry>;
  leaders?: Record<string, LeaderEntry>;
  annotation_styles?: Record<string, AnnotationStyleEntry>;
  families?: Record<string, FamilyEntry>;
  family_parameters?: Record<string, FamilyParameterEntry>;
  family_solids?: Record<string, FamilySolidEntry>;
  components?: Record<string, ComponentEntry>;
  component_overrides?: Record<string, ComponentOverrideEntry>;
  mep_elements?: Record<string, MepElementEntry>;
  clash_sets?: Record<string, ClashSetEntry>;
  rules?: Record<string, RuleEntry>;
  issues?: Record<string, IssueEntry>;
  issue_comments?: Record<string, IssueCommentEntry>;
  wall_sweeps?: Record<string, WallSweepEntry>;
  schedules?: Record<string, ScheduleEntry>;
  property_templates?: Record<string, PropertyTemplateEntry>;
  classification_systems?: Record<string, ClassificationSystemEntry>;
  properties?: Record<string, PropertySetEntry>;
  classifications?: Record<string, ClassificationSetEntry>;
}

import type {StructuralSupport,LoadCase,StructuralLoad,StructuralLocation,Restraints} from "../🟦️.ts";
export interface StructuralSupportPatch { name?: string; member?: string; location?: StructuralLocation; offset?: Point3; restraints?: Restraints; }
export type StructuralSupportEntry = ({entry:"Created"|"Replaced"} & StructuralSupport) | {entry:"Deleted"} | ({entry:"Patched"} & StructuralSupportPatch);
export interface LoadCasePatch { name?: string; category?: string; factor?: number; }
export type LoadCaseEntry = ({entry:"Created"|"Replaced"} & LoadCase) | {entry:"Deleted"} | ({entry:"Patched"} & LoadCasePatch);
export interface StructuralLoadPatch { name?: string; load_case?: string; member?: string; location?: StructuralLocation; force?: Point3; moment?: Point3; }
export type StructuralLoadEntry = ({entry:"Created"|"Replaced"} & StructuralLoad) | {entry:"Deleted"} | ({entry:"Patched"} & StructuralLoadPatch);
export interface ModelDiff { supports?:Record<string,StructuralSupportEntry>; load_cases?:Record<string,LoadCaseEntry>; loads?:Record<string,StructuralLoadEntry>; }
