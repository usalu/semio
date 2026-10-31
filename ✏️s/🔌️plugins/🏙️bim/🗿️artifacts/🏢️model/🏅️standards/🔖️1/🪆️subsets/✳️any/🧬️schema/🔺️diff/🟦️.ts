/** 🔺️ BIM model diff schema — a sparse per-collection keyed delta; absent collections are untouched. */
import { parseDesignOption, parseElementMembership, parseOptionGroup, parseWorkset } from "../🟦️.ts";
import { parseAnnotationAnchor, parseAnnotationStyle, parseAreaMeasure, parseAreaScheme, parseAxis, parseBaluster, parseBeam, parseBeamType, parseBuilding, parseCeiling, parseCeilingType, parseClashRef, parseClashSet, parseClassificationItem, parseClassificationSet, parseClassificationSystem, parseColumn, parseColumnType, parseComponent, parseComponentOverride, parseCurtainGrid, parseCurtainPanel, parseCurtainPanelOverride, parseCurtainWall, parseCurtainWallType, parseDetailLevel, parseDimension, parseDimensionUnit, parseDoorLeaves, parseDoorType, parseElementSelector, parseEndJoin, parseExprPoint3, parseFamily, parseFamilyCategory, parseFamilyParameter, parseFamilySolid, parseGridLine, parseInfill, parseIssue, parseIssueComment, parseIssuePriority, parseIssueStatus, parseIssueViewpoint, parseLayer, parseLeader, parseLocationLine, parseMaterial, parseMaterialCategory, parseMepElement, parseMepShape, parseMepSystem, parseOpening, parseOpeningKind, parseOrientation, parsePaper, parseParameterKind, parsePhase, parsePoint2, parsePoint3, parseProfile, parsePropertyDef, parsePropertySet, parsePropertyTemplate, parsePropertyValue, parseRailing, parseRailingHost, parseRamp, parseRestraints, parseRgb, parseRiserKind, parseRoof, parseRoofShape, parseRoofType, parseRule, parseRuleKind, parseRuleScope, parseRuleSeverity, parseSchedule, parseScheduleCategory, parseScheduleColumn, parseScheduleFilter, parseScheduleGroup, parseScheduleSort, parseSheet, parseSheetRevision, parseSite, parseSlab, parseSlabType, parseSlope, parseSolidShape, parseSpace, parseSpaceBoundary, parseSpaceConditions, parseStair, parseStairFlight, parseStairStringer, parseStorey, parseStructuralLocation, parseSwing, parseTag, parseTagCategory, parseTemplateTarget, parseTerminator, parseTextNote, parseTopConstraint, parseVertex, parseView, parseViewCamera, parseViewCategory, parseViewCrop, parseViewKind, parseViewPlane, parseViewport, parseWall, parseWallSide, parseWallSweep, parseWallType, parseWindowType, parseZone } from "../🟦️.ts";

import type { OptionGroup, DesignOption, Workset, ElementMembership, PropertyDef, ClassificationItem, ExprPoint, ExprPoint3, Point3, ElementSelector, RuleScope, ClashRef, SectionBox, IssueViewpoint, Point2, Rgb, Vertex, Slope, StairStringer, Baluster, RailingHost, Layer, ViewPlane, ViewCrop, ViewCamera, ScheduleColumn, ScheduleSort, ScheduleFilter, ScheduleGroup, AnnotationAnchor, ParametricProfile, SolidShape, MepShape, Paper, CurtainGrid, CurtainPanel, Axis, TopConstraint, Profile, RoofShape, OpeningKind, StairFlight, PropertyValue, Infill, SpaceBoundary, ScheduleKey, PropertyKind, TemplateTarget, AnchorEnd, WallSide, TagCategory, Terminator, DimensionUnit, FamilyCategory, ParameterKind, SolidAxis, MepSystem, ElementClass, RuleKind, RuleSeverity, IssueStatus, IssuePriority, IsoSize, Orientation, MaterialCategory, LayerFunction, LocationLine, Phase, DoorLeaves, Swing, Turn, StringerKind, RiserKind, ScheduleCategory, ScheduleField, ScheduleOp, EndJoin, ViewKind, DetailLevel, ViewCategory, HostSide, AreaMeasure, PropertyTemplate, ClassificationSystem, SpaceConditions, Dimension, Tag, TextNote, Leader, AnnotationStyle, Family, FamilyParameter, FamilySolid, Component, ComponentOverride, MepElement, ClashSet, Rule, Issue, IssueComment, Sheet, Viewport, SheetRevision, WallSweep, Material, WallType, SlabType, CeilingType, RoofType, ColumnType, BeamType, WindowType, DoorType, Site, Building, Storey, GridLine, Wall, CurtainWallType, CurtainPanelOverride, CurtainWall, Column, Beam, Slab, Ceiling, Roof, Opening, Stair, Railing, Ramp, Space, Zone, AreaScheme, View, Schedule, Project, PropertySet, ClassificationSet } from "../🟦️.ts";

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

export type MaterialEntry = { entry: "Created"; value: Material } | { entry: "Deleted" } | { entry: "Replaced"; value: Material } | { entry: "Patched"; value: MaterialPatch };
export type WallTypeEntry = { entry: "Created"; value: WallType } | { entry: "Deleted" } | { entry: "Replaced"; value: WallType } | { entry: "Patched"; value: WallTypePatch };
export type SlabTypeEntry = { entry: "Created"; value: SlabType } | { entry: "Deleted" } | { entry: "Replaced"; value: SlabType } | { entry: "Patched"; value: SlabTypePatch };
export type RoofTypeEntry = { entry: "Created"; value: RoofType } | { entry: "Deleted" } | { entry: "Replaced"; value: RoofType } | { entry: "Patched"; value: RoofTypePatch };
export type ColumnTypeEntry = { entry: "Created"; value: ColumnType } | { entry: "Deleted" } | { entry: "Replaced"; value: ColumnType } | { entry: "Patched"; value: ColumnTypePatch };
export type BeamTypeEntry = { entry: "Created"; value: BeamType } | { entry: "Deleted" } | { entry: "Replaced"; value: BeamType } | { entry: "Patched"; value: BeamTypePatch };
export type WindowTypeEntry = { entry: "Created"; value: WindowType } | { entry: "Deleted" } | { entry: "Replaced"; value: WindowType } | { entry: "Patched"; value: WindowTypePatch };
export type DoorTypeEntry = { entry: "Created"; value: DoorType } | { entry: "Deleted" } | { entry: "Replaced"; value: DoorType } | { entry: "Patched"; value: DoorTypePatch };
export type CurtainWallTypeEntry = { entry: "Created"; value: CurtainWallType } | { entry: "Deleted" } | { entry: "Replaced"; value: CurtainWallType } | { entry: "Patched"; value: CurtainWallTypePatch };
export type SiteEntry = { entry: "Created"; value: Site } | { entry: "Deleted" } | { entry: "Replaced"; value: Site } | { entry: "Patched"; value: SitePatch };
export type BuildingEntry = { entry: "Created"; value: Building } | { entry: "Deleted" } | { entry: "Replaced"; value: Building } | { entry: "Patched"; value: BuildingPatch };
export type StoreyEntry = { entry: "Created"; value: Storey } | { entry: "Deleted" } | { entry: "Replaced"; value: Storey } | { entry: "Patched"; value: StoreyPatch };
export type GridLineEntry = { entry: "Created"; value: GridLine } | { entry: "Deleted" } | { entry: "Replaced"; value: GridLine } | { entry: "Patched"; value: GridLinePatch };
export type WallEntry = { entry: "Created"; value: Wall } | { entry: "Deleted" } | { entry: "Replaced"; value: Wall } | { entry: "Patched"; value: WallPatch };
export type CurtainWallEntry = { entry: "Created"; value: CurtainWall } | { entry: "Deleted" } | { entry: "Replaced"; value: CurtainWall } | { entry: "Patched"; value: CurtainWallPatch };
export type CurtainPanelOverrideEntry = { entry: "Created"; value: CurtainPanelOverride } | { entry: "Deleted" } | { entry: "Replaced"; value: CurtainPanelOverride } | { entry: "Patched"; value: CurtainPanelOverridePatch };
export type ColumnEntry = { entry: "Created"; value: Column } | { entry: "Deleted" } | { entry: "Replaced"; value: Column } | { entry: "Patched"; value: ColumnPatch };
export type BeamEntry = { entry: "Created"; value: Beam } | { entry: "Deleted" } | { entry: "Replaced"; value: Beam } | { entry: "Patched"; value: BeamPatch };
export type SlabEntry = { entry: "Created"; value: Slab } | { entry: "Deleted" } | { entry: "Replaced"; value: Slab } | { entry: "Patched"; value: SlabPatch };
export type RoofEntry = { entry: "Created"; value: Roof } | { entry: "Deleted" } | { entry: "Replaced"; value: Roof } | { entry: "Patched"; value: RoofPatch };
export type OpeningEntry = { entry: "Created"; value: Opening } | { entry: "Deleted" } | { entry: "Replaced"; value: Opening } | { entry: "Patched"; value: OpeningPatch };
export type StairEntry = { entry: "Created"; value: Stair } | { entry: "Deleted" } | { entry: "Replaced"; value: Stair } | { entry: "Patched"; value: StairPatch };
export type RailingEntry = { entry: "Created"; value: Railing } | { entry: "Deleted" } | { entry: "Replaced"; value: Railing } | { entry: "Patched"; value: RailingPatch };
export type RampEntry = { entry: "Created"; value: Ramp } | { entry: "Deleted" } | { entry: "Replaced"; value: Ramp } | { entry: "Patched"; value: RampPatch };
export type SpaceEntry = { entry: "Created"; value: Space } | { entry: "Deleted" } | { entry: "Replaced"; value: Space } | { entry: "Patched"; value: SpacePatch };
export type CeilingTypeEntry = { entry: "Created"; value: CeilingType } | { entry: "Deleted" } | { entry: "Replaced"; value: CeilingType } | { entry: "Patched"; value: CeilingTypePatch };
export type CeilingEntry = { entry: "Created"; value: Ceiling } | { entry: "Deleted" } | { entry: "Replaced"; value: Ceiling } | { entry: "Patched"; value: CeilingPatch };
export type ZoneEntry = { entry: "Created"; value: Zone } | { entry: "Deleted" } | { entry: "Replaced"; value: Zone } | { entry: "Patched"; value: ZonePatch };
export type AreaSchemeEntry = { entry: "Created"; value: AreaScheme } | { entry: "Deleted" } | { entry: "Replaced"; value: AreaScheme } | { entry: "Patched"; value: AreaSchemePatch };
export type SpaceConditionsEntry = { entry: "Created"; value: SpaceConditions } | { entry: "Deleted" } | { entry: "Replaced"; value: SpaceConditions } | { entry: "Patched"; value: SpaceConditionsPatch };
export type ViewEntry = { entry: "Created"; value: View } | { entry: "Deleted" } | { entry: "Replaced"; value: View } | { entry: "Patched"; value: ViewPatch };
export type SheetEntry = { entry: "Created"; value: Sheet } | { entry: "Deleted" } | { entry: "Replaced"; value: Sheet } | { entry: "Patched"; value: SheetPatch };
export type ViewportEntry = { entry: "Created"; value: Viewport } | { entry: "Deleted" } | { entry: "Replaced"; value: Viewport } | { entry: "Patched"; value: ViewportPatch };
export type SheetRevisionEntry = { entry: "Created"; value: SheetRevision } | { entry: "Deleted" } | { entry: "Replaced"; value: SheetRevision } | { entry: "Patched"; value: SheetRevisionPatch };
export type DimensionEntry = { entry: "Created"; value: Dimension } | { entry: "Deleted" } | { entry: "Replaced"; value: Dimension } | { entry: "Patched"; value: DimensionPatch };
export type TagEntry = { entry: "Created"; value: Tag } | { entry: "Deleted" } | { entry: "Replaced"; value: Tag } | { entry: "Patched"; value: TagPatch };
export type TextNoteEntry = { entry: "Created"; value: TextNote } | { entry: "Deleted" } | { entry: "Replaced"; value: TextNote } | { entry: "Patched"; value: TextNotePatch };
export type LeaderEntry = { entry: "Created"; value: Leader } | { entry: "Deleted" } | { entry: "Replaced"; value: Leader } | { entry: "Patched"; value: LeaderPatch };
export type AnnotationStyleEntry = { entry: "Created"; value: AnnotationStyle } | { entry: "Deleted" } | { entry: "Replaced"; value: AnnotationStyle } | { entry: "Patched"; value: AnnotationStylePatch };
export type FamilyEntry = { entry: "Created"; value: Family } | { entry: "Deleted" } | { entry: "Replaced"; value: Family } | { entry: "Patched"; value: FamilyPatch };
export type FamilyParameterEntry = { entry: "Created"; value: FamilyParameter } | { entry: "Deleted" } | { entry: "Replaced"; value: FamilyParameter } | { entry: "Patched"; value: FamilyParameterPatch };
export type FamilySolidEntry = { entry: "Created"; value: FamilySolid } | { entry: "Deleted" } | { entry: "Replaced"; value: FamilySolid } | { entry: "Patched"; value: FamilySolidPatch };
export type ComponentEntry = { entry: "Created"; value: Component } | { entry: "Deleted" } | { entry: "Replaced"; value: Component } | { entry: "Patched"; value: ComponentPatch };
export type ComponentOverrideEntry = { entry: "Created"; value: ComponentOverride } | { entry: "Deleted" } | { entry: "Replaced"; value: ComponentOverride } | { entry: "Patched"; value: ComponentOverridePatch };
export type MepElementEntry = { entry: "Created"; value: MepElement } | { entry: "Deleted" } | { entry: "Replaced"; value: MepElement } | { entry: "Patched"; value: MepElementPatch };
export type ClashSetEntry = { entry: "Created"; value: ClashSet } | { entry: "Deleted" } | { entry: "Replaced"; value: ClashSet } | { entry: "Patched"; value: ClashSetPatch };
export type RuleEntry = { entry: "Created"; value: Rule } | { entry: "Deleted" } | { entry: "Replaced"; value: Rule } | { entry: "Patched"; value: RulePatch };
export type IssueEntry = { entry: "Created"; value: Issue } | { entry: "Deleted" } | { entry: "Replaced"; value: Issue } | { entry: "Patched"; value: IssuePatch };
export type IssueCommentEntry = { entry: "Created"; value: IssueComment } | { entry: "Deleted" } | { entry: "Replaced"; value: IssueComment } | { entry: "Patched"; value: IssueCommentPatch };
export type WallSweepEntry = { entry: "Created"; value: WallSweep } | { entry: "Deleted" } | { entry: "Replaced"; value: WallSweep } | { entry: "Patched"; value: WallSweepPatch };
export type ScheduleEntry = { entry: "Created"; value: Schedule } | { entry: "Deleted" } | { entry: "Replaced"; value: Schedule } | { entry: "Patched"; value: SchedulePatch };
export type PropertyTemplateEntry = { entry: "Created"; value: PropertyTemplate } | { entry: "Deleted" } | { entry: "Replaced"; value: PropertyTemplate } | { entry: "Patched"; value: PropertyTemplatePatch };
export type ClassificationSystemEntry = { entry: "Created"; value: ClassificationSystem } | { entry: "Deleted" } | { entry: "Replaced"; value: ClassificationSystem } | { entry: "Patched"; value: ClassificationSystemPatch };
export type PropertySetEntry = { entry: "Created"; value: PropertySet } | { entry: "Deleted" } | { entry: "Replaced"; value: PropertySet } | { entry: "Patched"; value: PropertySetPatch };
export type ClassificationSetEntry = { entry: "Created"; value: ClassificationSet } | { entry: "Deleted" } | { entry: "Replaced"; value: ClassificationSet } | { entry: "Patched"; value: ClassificationSetPatch };

export interface OptionGroupPatch { name?: string; }
export type OptionGroupEntry = { entry: "Created"; value: OptionGroup } | { entry: "Deleted" } | { entry: "Replaced"; value: OptionGroup } | { entry: "Patched"; value: OptionGroupPatch };
export interface DesignOptionPatch { group?: string; name?: string; primary?: boolean; }
export type DesignOptionEntry = { entry: "Created"; value: DesignOption } | { entry: "Deleted" } | { entry: "Replaced"; value: DesignOption } | { entry: "Patched"; value: DesignOptionPatch };
export interface WorksetPatch { name?: string; default_visible?: boolean; }
export type WorksetEntry = { entry: "Created"; value: Workset } | { entry: "Deleted" } | { entry: "Replaced"; value: Workset } | { entry: "Patched"; value: WorksetPatch };
export interface ElementMembershipPatch { target?: string; }
export type ElementMembershipEntry = { entry: "Created"; value: ElementMembership } | { entry: "Deleted" } | { entry: "Replaced"; value: ElementMembership } | { entry: "Patched"; value: ElementMembershipPatch };

export interface ModelDiff {
  option_groups?: Record<string, OptionGroupEntry>;
  design_options?: Record<string, DesignOptionEntry>;
  worksets?: Record<string, WorksetEntry>;
  element_options?: Record<string, ElementMembershipEntry>;
  element_worksets?: Record<string, ElementMembershipEntry>;
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
export type StructuralSupportEntry = { entry: "Created"; value: StructuralSupport } | { entry: "Deleted" } | { entry: "Replaced"; value: StructuralSupport } | { entry: "Patched"; value: StructuralSupportPatch };
export interface LoadCasePatch { name?: string; category?: string; factor?: number; }
export type LoadCaseEntry = { entry: "Created"; value: LoadCase } | { entry: "Deleted" } | { entry: "Replaced"; value: LoadCase } | { entry: "Patched"; value: LoadCasePatch };
export interface StructuralLoadPatch { name?: string; load_case?: string; member?: string; location?: StructuralLocation; force?: Point3; moment?: Point3; }
export type StructuralLoadEntry = { entry: "Created"; value: StructuralLoad } | { entry: "Deleted" } | { entry: "Replaced"; value: StructuralLoad } | { entry: "Patched"; value: StructuralLoadPatch };
export interface ModelDiff { supports?:Record<string,StructuralSupportEntry>; load_cases?:Record<string,LoadCaseEntry>; loads?:Record<string,StructuralLoadEntry>; }

//#region 🚪️Parsers
/** 🛑️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class bimModelDiffGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const bimModelDiffGuardReject = (at: string, why: string): never => {
  throw new bimModelDiffGuardRefusal(at, why);
};

type bimModelDiffGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type bimModelDiffGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number; readonly exclusiveMinimum?: number; readonly exclusiveMaximum?: number };
type bimModelDiffGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const bimModelDiffGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : bimModelDiffGuardReject(at, "value is not an object");
export const bimModelDiffGuardClosed = (row: Readonly<Record<string, unknown>>, at: string, keys: readonly string[]): Readonly<Record<string, unknown>> => {
  for (const key of Object.keys(row)) if (!keys.includes(key)) bimModelDiffGuardReject(`${at}.${key}`, "unknown property");
  return row;
};
export const bimModelDiffGuardExactObject = (value: unknown, at: string, fields: readonly string[], required: readonly string[] = fields): Readonly<Record<string, unknown>> => {
  const row = bimModelDiffGuardClosed(bimModelDiffGuardObject(value, at), at, fields);
  for (const field of required) if (!Object.hasOwn(row, field)) bimModelDiffGuardReject(`${at}.${field}`, "field is missing");
  return row;
};
export const bimModelDiffGuardArray = (value: unknown, at: string, bounds: bimModelDiffGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return bimModelDiffGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) bimModelDiffGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) bimModelDiffGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const bimModelDiffGuardString = (value: unknown, at: string, bounds: bimModelDiffGuardTextBounds = {}): string => {
  if (typeof value !== "string") return bimModelDiffGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) bimModelDiffGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) bimModelDiffGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) bimModelDiffGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const bimModelDiffGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : bimModelDiffGuardReject(at, "value is not a boolean"));
export const bimModelDiffGuardNumber = (value: unknown, at: string, bounds: bimModelDiffGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return bimModelDiffGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) bimModelDiffGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) bimModelDiffGuardReject(at, `number is above ${bounds.maximum}`);
  if (bounds.exclusiveMinimum !== undefined && value <= bounds.exclusiveMinimum) bimModelDiffGuardReject(at, `number is not above ${bounds.exclusiveMinimum}`);
  if (bounds.exclusiveMaximum !== undefined && value >= bounds.exclusiveMaximum) bimModelDiffGuardReject(at, `number is not below ${bounds.exclusiveMaximum}`);
  return value;
};
export const bimModelDiffGuardInteger = (value: unknown, at: string, bounds: bimModelDiffGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? bimModelDiffGuardNumber(value, at, bounds) : bimModelDiffGuardReject(at, "value is not an integer");
export const bimModelDiffGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : bimModelDiffGuardReject(at, `value is not one of ${members.join(", ")}`);
export const bimModelDiffGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : bimModelDiffGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseAnnotationStyleEntry(value: unknown, at = "$"): AnnotationStyleEntry {
  return ((row0) => { switch (bimModelDiffGuardMember(row0["entry"], `${at}.entry`, ["Created", "Deleted", "Replaced", "Patched"] as const)) { case "Created": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Created", value: parseAnnotationStyle(row0["value"], `${at}.value`) }; case "Deleted": bimModelDiffGuardClosed(row0, `${at}`, ["entry"]); return { entry: "Deleted" }; case "Replaced": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Replaced", value: parseAnnotationStyle(row0["value"], `${at}.value`) }; case "Patched": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Patched", value: parseAnnotationStylePatch(row0["value"], `${at}.value`) }; } })(bimModelDiffGuardObject(value, `${at}`));
}

export function parseAnnotationStylePatch(value: unknown, at = "$"): AnnotationStylePatch {
  const row = bimModelDiffGuardClosed(bimModelDiffGuardObject(value, at), at, ["name", "text_height", "terminator", "unit", "precision", "mark_size", "gap", "overshoot"]);
  return {
    name: row["name"] === undefined ? undefined : bimModelDiffGuardString(row["name"], `${at}.name`),
    text_height: row["text_height"] === undefined ? undefined : bimModelDiffGuardNumber(row["text_height"], `${at}.text_height`),
    terminator: row["terminator"] === undefined ? undefined : parseTerminator(row["terminator"], `${at}.terminator`),
    unit: row["unit"] === undefined ? undefined : parseDimensionUnit(row["unit"], `${at}.unit`),
    precision: row["precision"] === undefined ? undefined : bimModelDiffGuardInteger(row["precision"], `${at}.precision`),
    mark_size: row["mark_size"] === undefined ? undefined : bimModelDiffGuardNumber(row["mark_size"], `${at}.mark_size`),
    gap: row["gap"] === undefined ? undefined : bimModelDiffGuardNumber(row["gap"], `${at}.gap`),
    overshoot: row["overshoot"] === undefined ? undefined : bimModelDiffGuardNumber(row["overshoot"], `${at}.overshoot`),
  };
}

export function parseAreaSchemeEntry(value: unknown, at = "$"): AreaSchemeEntry {
  return ((row0) => { switch (bimModelDiffGuardMember(row0["entry"], `${at}.entry`, ["Created", "Deleted", "Replaced", "Patched"] as const)) { case "Created": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Created", value: parseAreaScheme(row0["value"], `${at}.value`) }; case "Deleted": bimModelDiffGuardClosed(row0, `${at}`, ["entry"]); return { entry: "Deleted" }; case "Replaced": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Replaced", value: parseAreaScheme(row0["value"], `${at}.value`) }; case "Patched": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Patched", value: parseAreaSchemePatch(row0["value"], `${at}.value`) }; } })(bimModelDiffGuardObject(value, `${at}`));
}

export function parseAreaSchemePatch(value: unknown, at = "$"): AreaSchemePatch {
  const row = bimModelDiffGuardClosed(bimModelDiffGuardObject(value, at), at, ["name", "measure", "usages", "zones"]);
  return {
    name: row["name"] === undefined ? undefined : bimModelDiffGuardString(row["name"], `${at}.name`),
    measure: row["measure"] === undefined ? undefined : parseAreaMeasure(row["measure"], `${at}.measure`),
    usages: row["usages"] === undefined ? undefined : bimModelDiffGuardArray(row["usages"], `${at}.usages`).map((item, index) => bimModelDiffGuardString(item, `${at}.usages[${index}]`)),
    zones: row["zones"] === undefined ? undefined : bimModelDiffGuardArray(row["zones"], `${at}.zones`).map((item, index) => bimModelDiffGuardString(item, `${at}.zones[${index}]`)),
  };
}

export function parseBeamEntry(value: unknown, at = "$"): BeamEntry {
  return ((row0) => { switch (bimModelDiffGuardMember(row0["entry"], `${at}.entry`, ["Created", "Deleted", "Replaced", "Patched"] as const)) { case "Created": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Created", value: parseBeam(row0["value"], `${at}.value`) }; case "Deleted": bimModelDiffGuardClosed(row0, `${at}`, ["entry"]); return { entry: "Deleted" }; case "Replaced": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Replaced", value: parseBeam(row0["value"], `${at}.value`) }; case "Patched": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Patched", value: parseBeamPatch(row0["value"], `${at}.value`) }; } })(bimModelDiffGuardObject(value, `${at}`));
}

export function parseBeamPatch(value: unknown, at = "$"): BeamPatch {
  const row = bimModelDiffGuardClosed(bimModelDiffGuardObject(value, at), at, ["storey", "beam_type", "axis", "top_offset", "end_top_offset", "phase", "name"]);
  return {
    storey: row["storey"] === undefined ? undefined : bimModelDiffGuardString(row["storey"], `${at}.storey`),
    beam_type: row["beam_type"] === undefined ? undefined : bimModelDiffGuardString(row["beam_type"], `${at}.beam_type`),
    axis: row["axis"] === undefined ? undefined : parseAxis(row["axis"], `${at}.axis`),
    top_offset: row["top_offset"] === undefined ? undefined : bimModelDiffGuardNumber(row["top_offset"], `${at}.top_offset`),
    end_top_offset: row["end_top_offset"] === undefined ? undefined : ((row0) => ({ value: (row0["value"] === null ? null : bimModelDiffGuardNumber(row0["value"], `${at}.end_top_offset.value`)) }))(bimModelDiffGuardClosed(bimModelDiffGuardObject(row["end_top_offset"], `${at}.end_top_offset`), `${at}.end_top_offset`, ["value"])),
    phase: row["phase"] === undefined ? undefined : parsePhase(row["phase"], `${at}.phase`),
    name: row["name"] === undefined ? undefined : bimModelDiffGuardString(row["name"], `${at}.name`),
  };
}

export function parseBeamTypeEntry(value: unknown, at = "$"): BeamTypeEntry {
  return ((row0) => { switch (bimModelDiffGuardMember(row0["entry"], `${at}.entry`, ["Created", "Deleted", "Replaced", "Patched"] as const)) { case "Created": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Created", value: parseBeamType(row0["value"], `${at}.value`) }; case "Deleted": bimModelDiffGuardClosed(row0, `${at}`, ["entry"]); return { entry: "Deleted" }; case "Replaced": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Replaced", value: parseBeamType(row0["value"], `${at}.value`) }; case "Patched": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Patched", value: parseBeamTypePatch(row0["value"], `${at}.value`) }; } })(bimModelDiffGuardObject(value, `${at}`));
}

export function parseBeamTypePatch(value: unknown, at = "$"): BeamTypePatch {
  const row = bimModelDiffGuardClosed(bimModelDiffGuardObject(value, at), at, ["name", "profile", "material"]);
  return {
    name: row["name"] === undefined ? undefined : bimModelDiffGuardString(row["name"], `${at}.name`),
    profile: row["profile"] === undefined ? undefined : parseProfile(row["profile"], `${at}.profile`),
    material: row["material"] === undefined ? undefined : bimModelDiffGuardString(row["material"], `${at}.material`),
  };
}

export function parseBuildingEntry(value: unknown, at = "$"): BuildingEntry {
  return ((row0) => { switch (bimModelDiffGuardMember(row0["entry"], `${at}.entry`, ["Created", "Deleted", "Replaced", "Patched"] as const)) { case "Created": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Created", value: parseBuilding(row0["value"], `${at}.value`) }; case "Deleted": bimModelDiffGuardClosed(row0, `${at}`, ["entry"]); return { entry: "Deleted" }; case "Replaced": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Replaced", value: parseBuilding(row0["value"], `${at}.value`) }; case "Patched": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Patched", value: parseBuildingPatch(row0["value"], `${at}.value`) }; } })(bimModelDiffGuardObject(value, `${at}`));
}

export function parseBuildingPatch(value: unknown, at = "$"): BuildingPatch {
  const row = bimModelDiffGuardClosed(bimModelDiffGuardObject(value, at), at, ["site", "name", "origin", "rotation", "elevation"]);
  return {
    site: row["site"] === undefined ? undefined : bimModelDiffGuardString(row["site"], `${at}.site`),
    name: row["name"] === undefined ? undefined : bimModelDiffGuardString(row["name"], `${at}.name`),
    origin: row["origin"] === undefined ? undefined : parsePoint2(row["origin"], `${at}.origin`),
    rotation: row["rotation"] === undefined ? undefined : bimModelDiffGuardNumber(row["rotation"], `${at}.rotation`),
    elevation: row["elevation"] === undefined ? undefined : bimModelDiffGuardNumber(row["elevation"], `${at}.elevation`),
  };
}

export function parseCeilingEntry(value: unknown, at = "$"): CeilingEntry {
  return ((row0) => { switch (bimModelDiffGuardMember(row0["entry"], `${at}.entry`, ["Created", "Deleted", "Replaced", "Patched"] as const)) { case "Created": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Created", value: parseCeiling(row0["value"], `${at}.value`) }; case "Deleted": bimModelDiffGuardClosed(row0, `${at}`, ["entry"]); return { entry: "Deleted" }; case "Replaced": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Replaced", value: parseCeiling(row0["value"], `${at}.value`) }; case "Patched": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Patched", value: parseCeilingPatch(row0["value"], `${at}.value`) }; } })(bimModelDiffGuardObject(value, `${at}`));
}

export function parseCeilingPatch(value: unknown, at = "$"): CeilingPatch {
  const row = bimModelDiffGuardClosed(bimModelDiffGuardObject(value, at), at, ["storey", "ceiling_type", "boundary", "holes", "offset", "slope", "name"]);
  return {
    storey: row["storey"] === undefined ? undefined : bimModelDiffGuardString(row["storey"], `${at}.storey`),
    ceiling_type: row["ceiling_type"] === undefined ? undefined : bimModelDiffGuardString(row["ceiling_type"], `${at}.ceiling_type`),
    boundary: row["boundary"] === undefined ? undefined : bimModelDiffGuardArray(row["boundary"], `${at}.boundary`).map((item, index) => parseVertex(item, `${at}.boundary[${index}]`)),
    holes: row["holes"] === undefined ? undefined : bimModelDiffGuardArray(row["holes"], `${at}.holes`).map((item, index) => bimModelDiffGuardArray(item, `${at}.holes[${index}]`).map((item1, index1) => parseVertex(item1, `${at}.holes[${index}][${index1}]`))),
    offset: row["offset"] === undefined ? undefined : bimModelDiffGuardNumber(row["offset"], `${at}.offset`),
    slope: row["slope"] === undefined ? undefined : ((row0) => ({ value: (row0["value"] === null ? null : parseSlope(row0["value"], `${at}.slope.value`)) }))(bimModelDiffGuardClosed(bimModelDiffGuardObject(row["slope"], `${at}.slope`), `${at}.slope`, ["value"])),
    name: row["name"] === undefined ? undefined : bimModelDiffGuardString(row["name"], `${at}.name`),
  };
}

export function parseCeilingTypeEntry(value: unknown, at = "$"): CeilingTypeEntry {
  return ((row0) => { switch (bimModelDiffGuardMember(row0["entry"], `${at}.entry`, ["Created", "Deleted", "Replaced", "Patched"] as const)) { case "Created": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Created", value: parseCeilingType(row0["value"], `${at}.value`) }; case "Deleted": bimModelDiffGuardClosed(row0, `${at}`, ["entry"]); return { entry: "Deleted" }; case "Replaced": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Replaced", value: parseCeilingType(row0["value"], `${at}.value`) }; case "Patched": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Patched", value: parseCeilingTypePatch(row0["value"], `${at}.value`) }; } })(bimModelDiffGuardObject(value, `${at}`));
}

export function parseCeilingTypePatch(value: unknown, at = "$"): CeilingTypePatch {
  const row = bimModelDiffGuardClosed(bimModelDiffGuardObject(value, at), at, ["name", "layers"]);
  return {
    name: row["name"] === undefined ? undefined : bimModelDiffGuardString(row["name"], `${at}.name`),
    layers: row["layers"] === undefined ? undefined : bimModelDiffGuardArray(row["layers"], `${at}.layers`).map((item, index) => parseLayer(item, `${at}.layers[${index}]`)),
  };
}

export function parseClashSetEntry(value: unknown, at = "$"): ClashSetEntry {
  return ((row0) => { switch (bimModelDiffGuardMember(row0["entry"], `${at}.entry`, ["Created", "Deleted", "Replaced", "Patched"] as const)) { case "Created": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Created", value: parseClashSet(row0["value"], `${at}.value`) }; case "Deleted": bimModelDiffGuardClosed(row0, `${at}`, ["entry"]); return { entry: "Deleted" }; case "Replaced": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Replaced", value: parseClashSet(row0["value"], `${at}.value`) }; case "Patched": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Patched", value: parseClashSetPatch(row0["value"], `${at}.value`) }; } })(bimModelDiffGuardObject(value, `${at}`));
}

export function parseClashSetPatch(value: unknown, at = "$"): ClashSetPatch {
  const row = bimModelDiffGuardClosed(bimModelDiffGuardObject(value, at), at, ["name", "a", "b", "tolerance", "clearance"]);
  return {
    name: row["name"] === undefined ? undefined : bimModelDiffGuardString(row["name"], `${at}.name`),
    a: row["a"] === undefined ? undefined : parseElementSelector(row["a"], `${at}.a`),
    b: row["b"] === undefined ? undefined : parseElementSelector(row["b"], `${at}.b`),
    tolerance: row["tolerance"] === undefined ? undefined : bimModelDiffGuardNumber(row["tolerance"], `${at}.tolerance`),
    clearance: row["clearance"] === undefined ? undefined : bimModelDiffGuardNumber(row["clearance"], `${at}.clearance`),
  };
}

export function parseClassificationSetEntry(value: unknown, at = "$"): ClassificationSetEntry {
  return ((row0) => { switch (bimModelDiffGuardMember(row0["entry"], `${at}.entry`, ["Created", "Deleted", "Replaced", "Patched"] as const)) { case "Created": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Created", value: parseClassificationSet(row0["value"], `${at}.value`) }; case "Deleted": bimModelDiffGuardClosed(row0, `${at}`, ["entry"]); return { entry: "Deleted" }; case "Replaced": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Replaced", value: parseClassificationSet(row0["value"], `${at}.value`) }; case "Patched": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Patched", value: parseClassificationSetPatch(row0["value"], `${at}.value`) }; } })(bimModelDiffGuardObject(value, `${at}`));
}

export function parseClassificationSetPatch(value: unknown, at = "$"): ClassificationSetPatch {
  const row = bimModelDiffGuardClosed(bimModelDiffGuardObject(value, at), at, ["assigned"]);
  return {
    assigned: row["assigned"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelDiffGuardObject(row["assigned"], `${at}.assigned`)).map(([key0, value0]) => [key0, (value0 === null ? null : bimModelDiffGuardString(value0, `${at}.assigned.${key0}`))])),
  };
}

export function parseClassificationSystemEntry(value: unknown, at = "$"): ClassificationSystemEntry {
  return ((row0) => { switch (bimModelDiffGuardMember(row0["entry"], `${at}.entry`, ["Created", "Deleted", "Replaced", "Patched"] as const)) { case "Created": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Created", value: parseClassificationSystem(row0["value"], `${at}.value`) }; case "Deleted": bimModelDiffGuardClosed(row0, `${at}`, ["entry"]); return { entry: "Deleted" }; case "Replaced": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Replaced", value: parseClassificationSystem(row0["value"], `${at}.value`) }; case "Patched": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Patched", value: parseClassificationSystemPatch(row0["value"], `${at}.value`) }; } })(bimModelDiffGuardObject(value, `${at}`));
}

export function parseClassificationSystemPatch(value: unknown, at = "$"): ClassificationSystemPatch {
  const row = bimModelDiffGuardClosed(bimModelDiffGuardObject(value, at), at, ["name", "edition", "source", "entries"]);
  return {
    name: row["name"] === undefined ? undefined : bimModelDiffGuardString(row["name"], `${at}.name`),
    edition: row["edition"] === undefined ? undefined : bimModelDiffGuardString(row["edition"], `${at}.edition`),
    source: row["source"] === undefined ? undefined : ((row0) => ({ value: (row0["value"] === null ? null : bimModelDiffGuardString(row0["value"], `${at}.source.value`)) }))(bimModelDiffGuardClosed(bimModelDiffGuardObject(row["source"], `${at}.source`), `${at}.source`, ["value"])),
    entries: row["entries"] === undefined ? undefined : bimModelDiffGuardArray(row["entries"], `${at}.entries`).map((item, index) => parseClassificationItem(item, `${at}.entries[${index}]`)),
  };
}

export function parseColumnEntry(value: unknown, at = "$"): ColumnEntry {
  return ((row0) => { switch (bimModelDiffGuardMember(row0["entry"], `${at}.entry`, ["Created", "Deleted", "Replaced", "Patched"] as const)) { case "Created": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Created", value: parseColumn(row0["value"], `${at}.value`) }; case "Deleted": bimModelDiffGuardClosed(row0, `${at}`, ["entry"]); return { entry: "Deleted" }; case "Replaced": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Replaced", value: parseColumn(row0["value"], `${at}.value`) }; case "Patched": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Patched", value: parseColumnPatch(row0["value"], `${at}.value`) }; } })(bimModelDiffGuardObject(value, `${at}`));
}

export function parseColumnPatch(value: unknown, at = "$"): ColumnPatch {
  const row = bimModelDiffGuardClosed(bimModelDiffGuardObject(value, at), at, ["storey", "column_type", "position", "rotation", "tilt", "base_offset", "top", "phase", "name"]);
  return {
    storey: row["storey"] === undefined ? undefined : bimModelDiffGuardString(row["storey"], `${at}.storey`),
    column_type: row["column_type"] === undefined ? undefined : bimModelDiffGuardString(row["column_type"], `${at}.column_type`),
    position: row["position"] === undefined ? undefined : parsePoint2(row["position"], `${at}.position`),
    rotation: row["rotation"] === undefined ? undefined : bimModelDiffGuardNumber(row["rotation"], `${at}.rotation`),
    tilt: row["tilt"] === undefined ? undefined : ((row0) => ({ value: (row0["value"] === null ? null : parseSlope(row0["value"], `${at}.tilt.value`)) }))(bimModelDiffGuardClosed(bimModelDiffGuardObject(row["tilt"], `${at}.tilt`), `${at}.tilt`, ["value"])),
    base_offset: row["base_offset"] === undefined ? undefined : bimModelDiffGuardNumber(row["base_offset"], `${at}.base_offset`),
    top: row["top"] === undefined ? undefined : parseTopConstraint(row["top"], `${at}.top`),
    phase: row["phase"] === undefined ? undefined : parsePhase(row["phase"], `${at}.phase`),
    name: row["name"] === undefined ? undefined : bimModelDiffGuardString(row["name"], `${at}.name`),
  };
}

export function parseColumnTypeEntry(value: unknown, at = "$"): ColumnTypeEntry {
  return ((row0) => { switch (bimModelDiffGuardMember(row0["entry"], `${at}.entry`, ["Created", "Deleted", "Replaced", "Patched"] as const)) { case "Created": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Created", value: parseColumnType(row0["value"], `${at}.value`) }; case "Deleted": bimModelDiffGuardClosed(row0, `${at}`, ["entry"]); return { entry: "Deleted" }; case "Replaced": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Replaced", value: parseColumnType(row0["value"], `${at}.value`) }; case "Patched": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Patched", value: parseColumnTypePatch(row0["value"], `${at}.value`) }; } })(bimModelDiffGuardObject(value, `${at}`));
}

export function parseColumnTypePatch(value: unknown, at = "$"): ColumnTypePatch {
  const row = bimModelDiffGuardClosed(bimModelDiffGuardObject(value, at), at, ["name", "profile", "material"]);
  return {
    name: row["name"] === undefined ? undefined : bimModelDiffGuardString(row["name"], `${at}.name`),
    profile: row["profile"] === undefined ? undefined : parseProfile(row["profile"], `${at}.profile`),
    material: row["material"] === undefined ? undefined : bimModelDiffGuardString(row["material"], `${at}.material`),
  };
}

export function parseComponentEntry(value: unknown, at = "$"): ComponentEntry {
  return ((row0) => { switch (bimModelDiffGuardMember(row0["entry"], `${at}.entry`, ["Created", "Deleted", "Replaced", "Patched"] as const)) { case "Created": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Created", value: parseComponent(row0["value"], `${at}.value`) }; case "Deleted": bimModelDiffGuardClosed(row0, `${at}`, ["entry"]); return { entry: "Deleted" }; case "Replaced": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Replaced", value: parseComponent(row0["value"], `${at}.value`) }; case "Patched": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Patched", value: parseComponentPatch(row0["value"], `${at}.value`) }; } })(bimModelDiffGuardObject(value, `${at}`));
}

export function parseComponentOverrideEntry(value: unknown, at = "$"): ComponentOverrideEntry {
  return ((row0) => { switch (bimModelDiffGuardMember(row0["entry"], `${at}.entry`, ["Created", "Deleted", "Replaced", "Patched"] as const)) { case "Created": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Created", value: parseComponentOverride(row0["value"], `${at}.value`) }; case "Deleted": bimModelDiffGuardClosed(row0, `${at}`, ["entry"]); return { entry: "Deleted" }; case "Replaced": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Replaced", value: parseComponentOverride(row0["value"], `${at}.value`) }; case "Patched": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Patched", value: parseComponentOverridePatch(row0["value"], `${at}.value`) }; } })(bimModelDiffGuardObject(value, `${at}`));
}

export function parseComponentOverridePatch(value: unknown, at = "$"): ComponentOverridePatch {
  const row = bimModelDiffGuardClosed(bimModelDiffGuardObject(value, at), at, ["component", "name", "value"]);
  return {
    component: row["component"] === undefined ? undefined : bimModelDiffGuardString(row["component"], `${at}.component`),
    name: row["name"] === undefined ? undefined : bimModelDiffGuardString(row["name"], `${at}.name`),
    value: row["value"] === undefined ? undefined : bimModelDiffGuardString(row["value"], `${at}.value`),
  };
}

export function parseComponentPatch(value: unknown, at = "$"): ComponentPatch {
  const row = bimModelDiffGuardClosed(bimModelDiffGuardObject(value, at), at, ["storey", "family", "position", "elevation", "rotation", "mirrored", "host", "system", "name"]);
  return {
    storey: row["storey"] === undefined ? undefined : bimModelDiffGuardString(row["storey"], `${at}.storey`),
    family: row["family"] === undefined ? undefined : bimModelDiffGuardString(row["family"], `${at}.family`),
    position: row["position"] === undefined ? undefined : parsePoint2(row["position"], `${at}.position`),
    elevation: row["elevation"] === undefined ? undefined : bimModelDiffGuardNumber(row["elevation"], `${at}.elevation`),
    rotation: row["rotation"] === undefined ? undefined : bimModelDiffGuardNumber(row["rotation"], `${at}.rotation`),
    mirrored: row["mirrored"] === undefined ? undefined : bimModelDiffGuardBoolean(row["mirrored"], `${at}.mirrored`),
    host: row["host"] === undefined ? undefined : ((row0) => ({ value: (row0["value"] === null ? null : bimModelDiffGuardString(row0["value"], `${at}.host.value`)) }))(bimModelDiffGuardClosed(bimModelDiffGuardObject(row["host"], `${at}.host`), `${at}.host`, ["value"])),
    system: row["system"] === undefined ? undefined : ((row0) => ({ value: (row0["value"] === null ? null : parseMepSystem(row0["value"], `${at}.system.value`)) }))(bimModelDiffGuardClosed(bimModelDiffGuardObject(row["system"], `${at}.system`), `${at}.system`, ["value"])),
    name: row["name"] === undefined ? undefined : bimModelDiffGuardString(row["name"], `${at}.name`),
  };
}

export function parseCurtainPanelOverrideEntry(value: unknown, at = "$"): CurtainPanelOverrideEntry {
  return ((row0) => { switch (bimModelDiffGuardMember(row0["entry"], `${at}.entry`, ["Created", "Deleted", "Replaced", "Patched"] as const)) { case "Created": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Created", value: parseCurtainPanelOverride(row0["value"], `${at}.value`) }; case "Deleted": bimModelDiffGuardClosed(row0, `${at}`, ["entry"]); return { entry: "Deleted" }; case "Replaced": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Replaced", value: parseCurtainPanelOverride(row0["value"], `${at}.value`) }; case "Patched": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Patched", value: parseCurtainPanelOverridePatch(row0["value"], `${at}.value`) }; } })(bimModelDiffGuardObject(value, `${at}`));
}

export function parseCurtainPanelOverridePatch(value: unknown, at = "$"): CurtainPanelOverridePatch {
  const row = bimModelDiffGuardClosed(bimModelDiffGuardObject(value, at), at, ["curtain", "u", "v", "panel"]);
  return {
    curtain: row["curtain"] === undefined ? undefined : bimModelDiffGuardString(row["curtain"], `${at}.curtain`),
    u: row["u"] === undefined ? undefined : bimModelDiffGuardInteger(row["u"], `${at}.u`),
    v: row["v"] === undefined ? undefined : bimModelDiffGuardInteger(row["v"], `${at}.v`),
    panel: row["panel"] === undefined ? undefined : parseCurtainPanel(row["panel"], `${at}.panel`),
  };
}

export function parseCurtainWallEntry(value: unknown, at = "$"): CurtainWallEntry {
  return ((row0) => { switch (bimModelDiffGuardMember(row0["entry"], `${at}.entry`, ["Created", "Deleted", "Replaced", "Patched"] as const)) { case "Created": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Created", value: parseCurtainWall(row0["value"], `${at}.value`) }; case "Deleted": bimModelDiffGuardClosed(row0, `${at}`, ["entry"]); return { entry: "Deleted" }; case "Replaced": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Replaced", value: parseCurtainWall(row0["value"], `${at}.value`) }; case "Patched": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Patched", value: parseCurtainWallPatch(row0["value"], `${at}.value`) }; } })(bimModelDiffGuardObject(value, `${at}`));
}

export function parseCurtainWallPatch(value: unknown, at = "$"): CurtainWallPatch {
  const row = bimModelDiffGuardClosed(bimModelDiffGuardObject(value, at), at, ["storey", "curtain_wall_type", "axis", "base_offset", "top", "u_grid", "v_grid", "phase", "name"]);
  return {
    storey: row["storey"] === undefined ? undefined : bimModelDiffGuardString(row["storey"], `${at}.storey`),
    curtain_wall_type: row["curtain_wall_type"] === undefined ? undefined : bimModelDiffGuardString(row["curtain_wall_type"], `${at}.curtain_wall_type`),
    axis: row["axis"] === undefined ? undefined : parseAxis(row["axis"], `${at}.axis`),
    base_offset: row["base_offset"] === undefined ? undefined : bimModelDiffGuardNumber(row["base_offset"], `${at}.base_offset`),
    top: row["top"] === undefined ? undefined : parseTopConstraint(row["top"], `${at}.top`),
    u_grid: row["u_grid"] === undefined ? undefined : ((row0) => ({ value: (row0["value"] === null ? null : parseCurtainGrid(row0["value"], `${at}.u_grid.value`)) }))(bimModelDiffGuardClosed(bimModelDiffGuardObject(row["u_grid"], `${at}.u_grid`), `${at}.u_grid`, ["value"])),
    v_grid: row["v_grid"] === undefined ? undefined : ((row0) => ({ value: (row0["value"] === null ? null : parseCurtainGrid(row0["value"], `${at}.v_grid.value`)) }))(bimModelDiffGuardClosed(bimModelDiffGuardObject(row["v_grid"], `${at}.v_grid`), `${at}.v_grid`, ["value"])),
    phase: row["phase"] === undefined ? undefined : parsePhase(row["phase"], `${at}.phase`),
    name: row["name"] === undefined ? undefined : bimModelDiffGuardString(row["name"], `${at}.name`),
  };
}

export function parseCurtainWallTypeEntry(value: unknown, at = "$"): CurtainWallTypeEntry {
  return ((row0) => { switch (bimModelDiffGuardMember(row0["entry"], `${at}.entry`, ["Created", "Deleted", "Replaced", "Patched"] as const)) { case "Created": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Created", value: parseCurtainWallType(row0["value"], `${at}.value`) }; case "Deleted": bimModelDiffGuardClosed(row0, `${at}`, ["entry"]); return { entry: "Deleted" }; case "Replaced": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Replaced", value: parseCurtainWallType(row0["value"], `${at}.value`) }; case "Patched": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Patched", value: parseCurtainWallTypePatch(row0["value"], `${at}.value`) }; } })(bimModelDiffGuardObject(value, `${at}`));
}

export function parseCurtainWallTypePatch(value: unknown, at = "$"): CurtainWallTypePatch {
  const row = bimModelDiffGuardClosed(bimModelDiffGuardObject(value, at), at, ["name", "u_grid", "v_grid", "interior_mullion", "border_mullion", "panel", "panel_material", "mullion_material", "u_value", "g_value", "frame_fraction"]);
  return {
    name: row["name"] === undefined ? undefined : bimModelDiffGuardString(row["name"], `${at}.name`),
    u_grid: row["u_grid"] === undefined ? undefined : parseCurtainGrid(row["u_grid"], `${at}.u_grid`),
    v_grid: row["v_grid"] === undefined ? undefined : parseCurtainGrid(row["v_grid"], `${at}.v_grid`),
    interior_mullion: row["interior_mullion"] === undefined ? undefined : parseProfile(row["interior_mullion"], `${at}.interior_mullion`),
    border_mullion: row["border_mullion"] === undefined ? undefined : parseProfile(row["border_mullion"], `${at}.border_mullion`),
    panel: row["panel"] === undefined ? undefined : parseCurtainPanel(row["panel"], `${at}.panel`),
    panel_material: row["panel_material"] === undefined ? undefined : bimModelDiffGuardString(row["panel_material"], `${at}.panel_material`),
    mullion_material: row["mullion_material"] === undefined ? undefined : bimModelDiffGuardString(row["mullion_material"], `${at}.mullion_material`),
    u_value: row["u_value"] === undefined ? undefined : ((row0) => ({ value: (row0["value"] === null ? null : bimModelDiffGuardNumber(row0["value"], `${at}.u_value.value`)) }))(bimModelDiffGuardClosed(bimModelDiffGuardObject(row["u_value"], `${at}.u_value`), `${at}.u_value`, ["value"])),
    g_value: row["g_value"] === undefined ? undefined : ((row0) => ({ value: (row0["value"] === null ? null : bimModelDiffGuardNumber(row0["value"], `${at}.g_value.value`)) }))(bimModelDiffGuardClosed(bimModelDiffGuardObject(row["g_value"], `${at}.g_value`), `${at}.g_value`, ["value"])),
    frame_fraction: row["frame_fraction"] === undefined ? undefined : ((row0) => ({ value: (row0["value"] === null ? null : bimModelDiffGuardNumber(row0["value"], `${at}.frame_fraction.value`)) }))(bimModelDiffGuardClosed(bimModelDiffGuardObject(row["frame_fraction"], `${at}.frame_fraction`), `${at}.frame_fraction`, ["value"])),
  };
}

export function parseDimensionEntry(value: unknown, at = "$"): DimensionEntry {
  return ((row0) => { switch (bimModelDiffGuardMember(row0["entry"], `${at}.entry`, ["Created", "Deleted", "Replaced", "Patched"] as const)) { case "Created": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Created", value: parseDimension(row0["value"], `${at}.value`) }; case "Deleted": bimModelDiffGuardClosed(row0, `${at}`, ["entry"]); return { entry: "Deleted" }; case "Replaced": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Replaced", value: parseDimension(row0["value"], `${at}.value`) }; case "Patched": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Patched", value: parseDimensionPatch(row0["value"], `${at}.value`) }; } })(bimModelDiffGuardObject(value, `${at}`));
}

export function parseDimensionPatch(value: unknown, at = "$"): DimensionPatch {
  const row = bimModelDiffGuardClosed(bimModelDiffGuardObject(value, at), at, ["storey", "anchors", "angle", "offset", "style", "lock", "name"]);
  return {
    storey: row["storey"] === undefined ? undefined : bimModelDiffGuardString(row["storey"], `${at}.storey`),
    anchors: row["anchors"] === undefined ? undefined : bimModelDiffGuardArray(row["anchors"], `${at}.anchors`).map((item, index) => parseAnnotationAnchor(item, `${at}.anchors[${index}]`)),
    angle: row["angle"] === undefined ? undefined : bimModelDiffGuardNumber(row["angle"], `${at}.angle`),
    offset: row["offset"] === undefined ? undefined : bimModelDiffGuardNumber(row["offset"], `${at}.offset`),
    style: row["style"] === undefined ? undefined : bimModelDiffGuardString(row["style"], `${at}.style`),
    lock: row["lock"] === undefined ? undefined : ((row0) => ({ value: (row0["value"] === null ? null : bimModelDiffGuardNumber(row0["value"], `${at}.lock.value`)) }))(bimModelDiffGuardClosed(bimModelDiffGuardObject(row["lock"], `${at}.lock`), `${at}.lock`, ["value"])),
    name: row["name"] === undefined ? undefined : bimModelDiffGuardString(row["name"], `${at}.name`),
  };
}

export function parseDoorTypeEntry(value: unknown, at = "$"): DoorTypeEntry {
  return ((row0) => { switch (bimModelDiffGuardMember(row0["entry"], `${at}.entry`, ["Created", "Deleted", "Replaced", "Patched"] as const)) { case "Created": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Created", value: parseDoorType(row0["value"], `${at}.value`) }; case "Deleted": bimModelDiffGuardClosed(row0, `${at}`, ["entry"]); return { entry: "Deleted" }; case "Replaced": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Replaced", value: parseDoorType(row0["value"], `${at}.value`) }; case "Patched": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Patched", value: parseDoorTypePatch(row0["value"], `${at}.value`) }; } })(bimModelDiffGuardObject(value, `${at}`));
}

export function parseDoorTypePatch(value: unknown, at = "$"): DoorTypePatch {
  const row = bimModelDiffGuardClosed(bimModelDiffGuardObject(value, at), at, ["name", "width", "height", "frame_width", "frame_depth", "leaves", "swing", "material", "u_value"]);
  return {
    name: row["name"] === undefined ? undefined : bimModelDiffGuardString(row["name"], `${at}.name`),
    width: row["width"] === undefined ? undefined : bimModelDiffGuardNumber(row["width"], `${at}.width`),
    height: row["height"] === undefined ? undefined : bimModelDiffGuardNumber(row["height"], `${at}.height`),
    frame_width: row["frame_width"] === undefined ? undefined : bimModelDiffGuardNumber(row["frame_width"], `${at}.frame_width`),
    frame_depth: row["frame_depth"] === undefined ? undefined : bimModelDiffGuardNumber(row["frame_depth"], `${at}.frame_depth`),
    leaves: row["leaves"] === undefined ? undefined : parseDoorLeaves(row["leaves"], `${at}.leaves`),
    swing: row["swing"] === undefined ? undefined : parseSwing(row["swing"], `${at}.swing`),
    material: row["material"] === undefined ? undefined : bimModelDiffGuardString(row["material"], `${at}.material`),
    u_value: row["u_value"] === undefined ? undefined : ((row0) => ({ value: (row0["value"] === null ? null : bimModelDiffGuardNumber(row0["value"], `${at}.u_value.value`)) }))(bimModelDiffGuardClosed(bimModelDiffGuardObject(row["u_value"], `${at}.u_value`), `${at}.u_value`, ["value"])),
  };
}

export function parseFamilyEntry(value: unknown, at = "$"): FamilyEntry {
  return ((row0) => { switch (bimModelDiffGuardMember(row0["entry"], `${at}.entry`, ["Created", "Deleted", "Replaced", "Patched"] as const)) { case "Created": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Created", value: parseFamily(row0["value"], `${at}.value`) }; case "Deleted": bimModelDiffGuardClosed(row0, `${at}`, ["entry"]); return { entry: "Deleted" }; case "Replaced": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Replaced", value: parseFamily(row0["value"], `${at}.value`) }; case "Patched": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Patched", value: parseFamilyPatch(row0["value"], `${at}.value`) }; } })(bimModelDiffGuardObject(value, `${at}`));
}

export function parseFamilyParameterEntry(value: unknown, at = "$"): FamilyParameterEntry {
  return ((row0) => { switch (bimModelDiffGuardMember(row0["entry"], `${at}.entry`, ["Created", "Deleted", "Replaced", "Patched"] as const)) { case "Created": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Created", value: parseFamilyParameter(row0["value"], `${at}.value`) }; case "Deleted": bimModelDiffGuardClosed(row0, `${at}`, ["entry"]); return { entry: "Deleted" }; case "Replaced": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Replaced", value: parseFamilyParameter(row0["value"], `${at}.value`) }; case "Patched": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Patched", value: parseFamilyParameterPatch(row0["value"], `${at}.value`) }; } })(bimModelDiffGuardObject(value, `${at}`));
}

export function parseFamilyParameterPatch(value: unknown, at = "$"): FamilyParameterPatch {
  const row = bimModelDiffGuardClosed(bimModelDiffGuardObject(value, at), at, ["family", "name", "kind", "value"]);
  return {
    family: row["family"] === undefined ? undefined : bimModelDiffGuardString(row["family"], `${at}.family`),
    name: row["name"] === undefined ? undefined : bimModelDiffGuardString(row["name"], `${at}.name`),
    kind: row["kind"] === undefined ? undefined : parseParameterKind(row["kind"], `${at}.kind`),
    value: row["value"] === undefined ? undefined : bimModelDiffGuardString(row["value"], `${at}.value`),
  };
}

export function parseFamilyPatch(value: unknown, at = "$"): FamilyPatch {
  const row = bimModelDiffGuardClosed(bimModelDiffGuardObject(value, at), at, ["name", "category"]);
  return {
    name: row["name"] === undefined ? undefined : bimModelDiffGuardString(row["name"], `${at}.name`),
    category: row["category"] === undefined ? undefined : parseFamilyCategory(row["category"], `${at}.category`),
  };
}

export function parseFamilySolidEntry(value: unknown, at = "$"): FamilySolidEntry {
  return ((row0) => { switch (bimModelDiffGuardMember(row0["entry"], `${at}.entry`, ["Created", "Deleted", "Replaced", "Patched"] as const)) { case "Created": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Created", value: parseFamilySolid(row0["value"], `${at}.value`) }; case "Deleted": bimModelDiffGuardClosed(row0, `${at}`, ["entry"]); return { entry: "Deleted" }; case "Replaced": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Replaced", value: parseFamilySolid(row0["value"], `${at}.value`) }; case "Patched": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Patched", value: parseFamilySolidPatch(row0["value"], `${at}.value`) }; } })(bimModelDiffGuardObject(value, `${at}`));
}

export function parseFamilySolidPatch(value: unknown, at = "$"): FamilySolidPatch {
  const row = bimModelDiffGuardClosed(bimModelDiffGuardObject(value, at), at, ["family", "name", "shape", "material", "visible", "offset"]);
  return {
    family: row["family"] === undefined ? undefined : bimModelDiffGuardString(row["family"], `${at}.family`),
    name: row["name"] === undefined ? undefined : bimModelDiffGuardString(row["name"], `${at}.name`),
    shape: row["shape"] === undefined ? undefined : parseSolidShape(row["shape"], `${at}.shape`),
    material: row["material"] === undefined ? undefined : bimModelDiffGuardString(row["material"], `${at}.material`),
    visible: row["visible"] === undefined ? undefined : bimModelDiffGuardString(row["visible"], `${at}.visible`),
    offset: row["offset"] === undefined ? undefined : parseExprPoint3(row["offset"], `${at}.offset`),
  };
}

export function parseGridLineEntry(value: unknown, at = "$"): GridLineEntry {
  return ((row0) => { switch (bimModelDiffGuardMember(row0["entry"], `${at}.entry`, ["Created", "Deleted", "Replaced", "Patched"] as const)) { case "Created": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Created", value: parseGridLine(row0["value"], `${at}.value`) }; case "Deleted": bimModelDiffGuardClosed(row0, `${at}`, ["entry"]); return { entry: "Deleted" }; case "Replaced": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Replaced", value: parseGridLine(row0["value"], `${at}.value`) }; case "Patched": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Patched", value: parseGridLinePatch(row0["value"], `${at}.value`) }; } })(bimModelDiffGuardObject(value, `${at}`));
}

export function parseGridLinePatch(value: unknown, at = "$"): GridLinePatch {
  const row = bimModelDiffGuardClosed(bimModelDiffGuardObject(value, at), at, ["building", "label", "start", "end"]);
  return {
    building: row["building"] === undefined ? undefined : bimModelDiffGuardString(row["building"], `${at}.building`),
    label: row["label"] === undefined ? undefined : bimModelDiffGuardString(row["label"], `${at}.label`),
    start: row["start"] === undefined ? undefined : parsePoint2(row["start"], `${at}.start`),
    end: row["end"] === undefined ? undefined : parsePoint2(row["end"], `${at}.end`),
  };
}

export function parseIssueCommentEntry(value: unknown, at = "$"): IssueCommentEntry {
  return ((row0) => { switch (bimModelDiffGuardMember(row0["entry"], `${at}.entry`, ["Created", "Deleted", "Replaced", "Patched"] as const)) { case "Created": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Created", value: parseIssueComment(row0["value"], `${at}.value`) }; case "Deleted": bimModelDiffGuardClosed(row0, `${at}`, ["entry"]); return { entry: "Deleted" }; case "Replaced": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Replaced", value: parseIssueComment(row0["value"], `${at}.value`) }; case "Patched": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Patched", value: parseIssueCommentPatch(row0["value"], `${at}.value`) }; } })(bimModelDiffGuardObject(value, `${at}`));
}

export function parseIssueCommentPatch(value: unknown, at = "$"): IssueCommentPatch {
  const row = bimModelDiffGuardClosed(bimModelDiffGuardObject(value, at), at, ["issue", "author", "date", "text"]);
  return {
    issue: row["issue"] === undefined ? undefined : bimModelDiffGuardString(row["issue"], `${at}.issue`),
    author: row["author"] === undefined ? undefined : bimModelDiffGuardString(row["author"], `${at}.author`),
    date: row["date"] === undefined ? undefined : bimModelDiffGuardString(row["date"], `${at}.date`),
    text: row["text"] === undefined ? undefined : bimModelDiffGuardString(row["text"], `${at}.text`),
  };
}

export function parseIssueEntry(value: unknown, at = "$"): IssueEntry {
  return ((row0) => { switch (bimModelDiffGuardMember(row0["entry"], `${at}.entry`, ["Created", "Deleted", "Replaced", "Patched"] as const)) { case "Created": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Created", value: parseIssue(row0["value"], `${at}.value`) }; case "Deleted": bimModelDiffGuardClosed(row0, `${at}`, ["entry"]); return { entry: "Deleted" }; case "Replaced": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Replaced", value: parseIssue(row0["value"], `${at}.value`) }; case "Patched": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Patched", value: parseIssuePatch(row0["value"], `${at}.value`) }; } })(bimModelDiffGuardObject(value, `${at}`));
}

export function parseIssuePatch(value: unknown, at = "$"): IssuePatch {
  const row = bimModelDiffGuardClosed(bimModelDiffGuardObject(value, at), at, ["title", "description", "status", "priority", "assignee", "author", "created", "labels", "elements", "clash", "viewpoint"]);
  return {
    title: row["title"] === undefined ? undefined : bimModelDiffGuardString(row["title"], `${at}.title`),
    description: row["description"] === undefined ? undefined : bimModelDiffGuardString(row["description"], `${at}.description`),
    status: row["status"] === undefined ? undefined : parseIssueStatus(row["status"], `${at}.status`),
    priority: row["priority"] === undefined ? undefined : parseIssuePriority(row["priority"], `${at}.priority`),
    assignee: row["assignee"] === undefined ? undefined : bimModelDiffGuardString(row["assignee"], `${at}.assignee`),
    author: row["author"] === undefined ? undefined : bimModelDiffGuardString(row["author"], `${at}.author`),
    created: row["created"] === undefined ? undefined : bimModelDiffGuardString(row["created"], `${at}.created`),
    labels: row["labels"] === undefined ? undefined : bimModelDiffGuardArray(row["labels"], `${at}.labels`).map((item, index) => bimModelDiffGuardString(item, `${at}.labels[${index}]`)),
    elements: row["elements"] === undefined ? undefined : bimModelDiffGuardArray(row["elements"], `${at}.elements`).map((item, index) => bimModelDiffGuardString(item, `${at}.elements[${index}]`)),
    clash: row["clash"] === undefined ? undefined : ((row0) => ({ value: (row0["value"] === null ? null : parseClashRef(row0["value"], `${at}.clash.value`)) }))(bimModelDiffGuardClosed(bimModelDiffGuardObject(row["clash"], `${at}.clash`), `${at}.clash`, ["value"])),
    viewpoint: row["viewpoint"] === undefined ? undefined : ((row0) => ({ value: (row0["value"] === null ? null : parseIssueViewpoint(row0["value"], `${at}.viewpoint.value`)) }))(bimModelDiffGuardClosed(bimModelDiffGuardObject(row["viewpoint"], `${at}.viewpoint`), `${at}.viewpoint`, ["value"])),
  };
}

export function parseLeaderEntry(value: unknown, at = "$"): LeaderEntry {
  return ((row0) => { switch (bimModelDiffGuardMember(row0["entry"], `${at}.entry`, ["Created", "Deleted", "Replaced", "Patched"] as const)) { case "Created": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Created", value: parseLeader(row0["value"], `${at}.value`) }; case "Deleted": bimModelDiffGuardClosed(row0, `${at}`, ["entry"]); return { entry: "Deleted" }; case "Replaced": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Replaced", value: parseLeader(row0["value"], `${at}.value`) }; case "Patched": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Patched", value: parseLeaderPatch(row0["value"], `${at}.value`) }; } })(bimModelDiffGuardObject(value, `${at}`));
}

export function parseLeaderPatch(value: unknown, at = "$"): LeaderPatch {
  const row = bimModelDiffGuardClosed(bimModelDiffGuardObject(value, at), at, ["storey", "anchor", "offset", "text", "style"]);
  return {
    storey: row["storey"] === undefined ? undefined : bimModelDiffGuardString(row["storey"], `${at}.storey`),
    anchor: row["anchor"] === undefined ? undefined : parseAnnotationAnchor(row["anchor"], `${at}.anchor`),
    offset: row["offset"] === undefined ? undefined : parsePoint2(row["offset"], `${at}.offset`),
    text: row["text"] === undefined ? undefined : bimModelDiffGuardString(row["text"], `${at}.text`),
    style: row["style"] === undefined ? undefined : bimModelDiffGuardString(row["style"], `${at}.style`),
  };
}

export function parseLoadCaseEntry(value: unknown, at = "$"): LoadCaseEntry {
  return ((row0) => { switch (bimModelDiffGuardMember(row0["entry"], `${at}.entry`, ["Created", "Replaced", "Deleted", "Patched"] as const)) { case "Created": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Created", value: ((row1) => ({ name: bimModelDiffGuardString(row1["name"], `${at}.value.name`), category: bimModelDiffGuardString(row1["category"], `${at}.value.category`), factor: bimModelDiffGuardNumber(row1["factor"], `${at}.value.factor`) }))(bimModelDiffGuardClosed(bimModelDiffGuardObject(row0["value"], `${at}.value`), `${at}.value`, ["name", "category", "factor"])) }; case "Replaced": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Replaced", value: ((row1) => ({ name: bimModelDiffGuardString(row1["name"], `${at}.value.name`), category: bimModelDiffGuardString(row1["category"], `${at}.value.category`), factor: bimModelDiffGuardNumber(row1["factor"], `${at}.value.factor`) }))(bimModelDiffGuardClosed(bimModelDiffGuardObject(row0["value"], `${at}.value`), `${at}.value`, ["name", "category", "factor"])) }; case "Deleted": bimModelDiffGuardClosed(row0, `${at}`, ["entry"]); return { entry: "Deleted" }; case "Patched": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Patched", value: ((row1) => ({ name: row1["name"] === undefined ? undefined : bimModelDiffGuardString(row1["name"], `${at}.value.name`), category: row1["category"] === undefined ? undefined : bimModelDiffGuardString(row1["category"], `${at}.value.category`), factor: row1["factor"] === undefined ? undefined : bimModelDiffGuardNumber(row1["factor"], `${at}.value.factor`) }))(bimModelDiffGuardClosed(bimModelDiffGuardObject(row0["value"], `${at}.value`), `${at}.value`, ["name", "category", "factor"])) }; } })(bimModelDiffGuardObject(value, `${at}`));
}

export function parseLoadCasePatch(value: unknown, at = "$"): LoadCasePatch {
  const row = bimModelDiffGuardClosed(bimModelDiffGuardObject(value, at), at, ["name", "category", "factor"]);
  return {
    name: row["name"] === undefined ? undefined : bimModelDiffGuardString(row["name"], `${at}.name`),
    category: row["category"] === undefined ? undefined : bimModelDiffGuardString(row["category"], `${at}.category`),
    factor: row["factor"] === undefined ? undefined : bimModelDiffGuardNumber(row["factor"], `${at}.factor`),
  };
}

export function parseMaterialEntry(value: unknown, at = "$"): MaterialEntry {
  return ((row0) => { switch (bimModelDiffGuardMember(row0["entry"], `${at}.entry`, ["Created", "Deleted", "Replaced", "Patched"] as const)) { case "Created": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Created", value: parseMaterial(row0["value"], `${at}.value`) }; case "Deleted": bimModelDiffGuardClosed(row0, `${at}`, ["entry"]); return { entry: "Deleted" }; case "Replaced": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Replaced", value: parseMaterial(row0["value"], `${at}.value`) }; case "Patched": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Patched", value: parseMaterialPatch(row0["value"], `${at}.value`) }; } })(bimModelDiffGuardObject(value, `${at}`));
}

export function parseMaterialPatch(value: unknown, at = "$"): MaterialPatch {
  const row = bimModelDiffGuardClosed(bimModelDiffGuardObject(value, at), at, ["name", "category", "color", "density", "conductivity", "specific_heat"]);
  return {
    name: row["name"] === undefined ? undefined : bimModelDiffGuardString(row["name"], `${at}.name`),
    category: row["category"] === undefined ? undefined : parseMaterialCategory(row["category"], `${at}.category`),
    color: row["color"] === undefined ? undefined : parseRgb(row["color"], `${at}.color`),
    density: row["density"] === undefined ? undefined : bimModelDiffGuardNumber(row["density"], `${at}.density`),
    conductivity: row["conductivity"] === undefined ? undefined : bimModelDiffGuardNumber(row["conductivity"], `${at}.conductivity`),
    specific_heat: row["specific_heat"] === undefined ? undefined : bimModelDiffGuardNumber(row["specific_heat"], `${at}.specific_heat`),
  };
}

export function parseMepElementEntry(value: unknown, at = "$"): MepElementEntry {
  return ((row0) => { switch (bimModelDiffGuardMember(row0["entry"], `${at}.entry`, ["Created", "Deleted", "Replaced", "Patched"] as const)) { case "Created": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Created", value: parseMepElement(row0["value"], `${at}.value`) }; case "Deleted": bimModelDiffGuardClosed(row0, `${at}`, ["entry"]); return { entry: "Deleted" }; case "Replaced": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Replaced", value: parseMepElement(row0["value"], `${at}.value`) }; case "Patched": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Patched", value: parseMepElementPatch(row0["value"], `${at}.value`) }; } })(bimModelDiffGuardObject(value, `${at}`));
}

export function parseMepElementPatch(value: unknown, at = "$"): MepElementPatch {
  const row = bimModelDiffGuardClosed(bimModelDiffGuardObject(value, at), at, ["storey", "system", "shape", "path", "name"]);
  return {
    storey: row["storey"] === undefined ? undefined : bimModelDiffGuardString(row["storey"], `${at}.storey`),
    system: row["system"] === undefined ? undefined : parseMepSystem(row["system"], `${at}.system`),
    shape: row["shape"] === undefined ? undefined : parseMepShape(row["shape"], `${at}.shape`),
    path: row["path"] === undefined ? undefined : bimModelDiffGuardArray(row["path"], `${at}.path`).map((item, index) => parsePoint3(item, `${at}.path[${index}]`)),
    name: row["name"] === undefined ? undefined : bimModelDiffGuardString(row["name"], `${at}.name`),
  };
}

export function parseModelDiff(value: unknown, at = "$"): ModelDiff {
  const row = bimModelDiffGuardClosed(bimModelDiffGuardObject(value, at), at, ["project", "materials", "wall_types", "slab_types", "roof_types", "column_types", "beam_types", "window_types", "door_types", "curtain_wall_types", "sites", "buildings", "storeys", "grids", "walls", "curtain_walls", "curtain_panel_overrides", "columns", "beams", "slabs", "roofs", "openings", "stairs", "railings", "ramps", "spaces", "ceiling_types", "ceilings", "zones", "area_schemes", "space_conditions", "views", "sheets", "viewports", "sheet_revisions", "dimensions", "tags", "text_notes", "leaders", "annotation_styles", "families", "family_parameters", "family_solids", "components", "component_overrides", "mep_elements", "clash_sets", "rules", "issues", "issue_comments", "wall_sweeps", "schedules", "property_templates", "classification_systems", "properties", "classifications", "supports", "load_cases", "loads", "option_groups", "design_options", "worksets", "element_options", "element_worksets"]);
  return {
    project: row["project"] === undefined ? undefined : parseProjectPatch(row["project"], `${at}.project`),
    materials: row["materials"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelDiffGuardObject(row["materials"], `${at}.materials`)).map(([key0, value0]) => [key0, parseMaterialEntry(value0, `${at}.materials.${key0}`)])),
    wall_types: row["wall_types"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelDiffGuardObject(row["wall_types"], `${at}.wall_types`)).map(([key0, value0]) => [key0, parseWallTypeEntry(value0, `${at}.wall_types.${key0}`)])),
    slab_types: row["slab_types"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelDiffGuardObject(row["slab_types"], `${at}.slab_types`)).map(([key0, value0]) => [key0, parseSlabTypeEntry(value0, `${at}.slab_types.${key0}`)])),
    roof_types: row["roof_types"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelDiffGuardObject(row["roof_types"], `${at}.roof_types`)).map(([key0, value0]) => [key0, parseRoofTypeEntry(value0, `${at}.roof_types.${key0}`)])),
    column_types: row["column_types"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelDiffGuardObject(row["column_types"], `${at}.column_types`)).map(([key0, value0]) => [key0, parseColumnTypeEntry(value0, `${at}.column_types.${key0}`)])),
    beam_types: row["beam_types"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelDiffGuardObject(row["beam_types"], `${at}.beam_types`)).map(([key0, value0]) => [key0, parseBeamTypeEntry(value0, `${at}.beam_types.${key0}`)])),
    window_types: row["window_types"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelDiffGuardObject(row["window_types"], `${at}.window_types`)).map(([key0, value0]) => [key0, parseWindowTypeEntry(value0, `${at}.window_types.${key0}`)])),
    door_types: row["door_types"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelDiffGuardObject(row["door_types"], `${at}.door_types`)).map(([key0, value0]) => [key0, parseDoorTypeEntry(value0, `${at}.door_types.${key0}`)])),
    curtain_wall_types: row["curtain_wall_types"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelDiffGuardObject(row["curtain_wall_types"], `${at}.curtain_wall_types`)).map(([key0, value0]) => [key0, parseCurtainWallTypeEntry(value0, `${at}.curtain_wall_types.${key0}`)])),
    sites: row["sites"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelDiffGuardObject(row["sites"], `${at}.sites`)).map(([key0, value0]) => [key0, parseSiteEntry(value0, `${at}.sites.${key0}`)])),
    buildings: row["buildings"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelDiffGuardObject(row["buildings"], `${at}.buildings`)).map(([key0, value0]) => [key0, parseBuildingEntry(value0, `${at}.buildings.${key0}`)])),
    storeys: row["storeys"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelDiffGuardObject(row["storeys"], `${at}.storeys`)).map(([key0, value0]) => [key0, parseStoreyEntry(value0, `${at}.storeys.${key0}`)])),
    grids: row["grids"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelDiffGuardObject(row["grids"], `${at}.grids`)).map(([key0, value0]) => [key0, parseGridLineEntry(value0, `${at}.grids.${key0}`)])),
    walls: row["walls"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelDiffGuardObject(row["walls"], `${at}.walls`)).map(([key0, value0]) => [key0, parseWallEntry(value0, `${at}.walls.${key0}`)])),
    curtain_walls: row["curtain_walls"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelDiffGuardObject(row["curtain_walls"], `${at}.curtain_walls`)).map(([key0, value0]) => [key0, parseCurtainWallEntry(value0, `${at}.curtain_walls.${key0}`)])),
    curtain_panel_overrides: row["curtain_panel_overrides"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelDiffGuardObject(row["curtain_panel_overrides"], `${at}.curtain_panel_overrides`)).map(([key0, value0]) => [key0, parseCurtainPanelOverrideEntry(value0, `${at}.curtain_panel_overrides.${key0}`)])),
    columns: row["columns"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelDiffGuardObject(row["columns"], `${at}.columns`)).map(([key0, value0]) => [key0, parseColumnEntry(value0, `${at}.columns.${key0}`)])),
    beams: row["beams"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelDiffGuardObject(row["beams"], `${at}.beams`)).map(([key0, value0]) => [key0, parseBeamEntry(value0, `${at}.beams.${key0}`)])),
    slabs: row["slabs"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelDiffGuardObject(row["slabs"], `${at}.slabs`)).map(([key0, value0]) => [key0, parseSlabEntry(value0, `${at}.slabs.${key0}`)])),
    roofs: row["roofs"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelDiffGuardObject(row["roofs"], `${at}.roofs`)).map(([key0, value0]) => [key0, parseRoofEntry(value0, `${at}.roofs.${key0}`)])),
    openings: row["openings"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelDiffGuardObject(row["openings"], `${at}.openings`)).map(([key0, value0]) => [key0, parseOpeningEntry(value0, `${at}.openings.${key0}`)])),
    stairs: row["stairs"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelDiffGuardObject(row["stairs"], `${at}.stairs`)).map(([key0, value0]) => [key0, parseStairEntry(value0, `${at}.stairs.${key0}`)])),
    railings: row["railings"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelDiffGuardObject(row["railings"], `${at}.railings`)).map(([key0, value0]) => [key0, parseRailingEntry(value0, `${at}.railings.${key0}`)])),
    ramps: row["ramps"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelDiffGuardObject(row["ramps"], `${at}.ramps`)).map(([key0, value0]) => [key0, parseRampEntry(value0, `${at}.ramps.${key0}`)])),
    spaces: row["spaces"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelDiffGuardObject(row["spaces"], `${at}.spaces`)).map(([key0, value0]) => [key0, parseSpaceEntry(value0, `${at}.spaces.${key0}`)])),
    ceiling_types: row["ceiling_types"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelDiffGuardObject(row["ceiling_types"], `${at}.ceiling_types`)).map(([key0, value0]) => [key0, parseCeilingTypeEntry(value0, `${at}.ceiling_types.${key0}`)])),
    ceilings: row["ceilings"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelDiffGuardObject(row["ceilings"], `${at}.ceilings`)).map(([key0, value0]) => [key0, parseCeilingEntry(value0, `${at}.ceilings.${key0}`)])),
    zones: row["zones"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelDiffGuardObject(row["zones"], `${at}.zones`)).map(([key0, value0]) => [key0, parseZoneEntry(value0, `${at}.zones.${key0}`)])),
    area_schemes: row["area_schemes"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelDiffGuardObject(row["area_schemes"], `${at}.area_schemes`)).map(([key0, value0]) => [key0, parseAreaSchemeEntry(value0, `${at}.area_schemes.${key0}`)])),
    space_conditions: row["space_conditions"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelDiffGuardObject(row["space_conditions"], `${at}.space_conditions`)).map(([key0, value0]) => [key0, parseSpaceConditionsEntry(value0, `${at}.space_conditions.${key0}`)])),
    views: row["views"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelDiffGuardObject(row["views"], `${at}.views`)).map(([key0, value0]) => [key0, parseViewEntry(value0, `${at}.views.${key0}`)])),
    sheets: row["sheets"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelDiffGuardObject(row["sheets"], `${at}.sheets`)).map(([key0, value0]) => [key0, parseSheetEntry(value0, `${at}.sheets.${key0}`)])),
    viewports: row["viewports"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelDiffGuardObject(row["viewports"], `${at}.viewports`)).map(([key0, value0]) => [key0, parseViewportEntry(value0, `${at}.viewports.${key0}`)])),
    sheet_revisions: row["sheet_revisions"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelDiffGuardObject(row["sheet_revisions"], `${at}.sheet_revisions`)).map(([key0, value0]) => [key0, parseSheetRevisionEntry(value0, `${at}.sheet_revisions.${key0}`)])),
    dimensions: row["dimensions"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelDiffGuardObject(row["dimensions"], `${at}.dimensions`)).map(([key0, value0]) => [key0, parseDimensionEntry(value0, `${at}.dimensions.${key0}`)])),
    tags: row["tags"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelDiffGuardObject(row["tags"], `${at}.tags`)).map(([key0, value0]) => [key0, parseTagEntry(value0, `${at}.tags.${key0}`)])),
    text_notes: row["text_notes"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelDiffGuardObject(row["text_notes"], `${at}.text_notes`)).map(([key0, value0]) => [key0, parseTextNoteEntry(value0, `${at}.text_notes.${key0}`)])),
    leaders: row["leaders"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelDiffGuardObject(row["leaders"], `${at}.leaders`)).map(([key0, value0]) => [key0, parseLeaderEntry(value0, `${at}.leaders.${key0}`)])),
    annotation_styles: row["annotation_styles"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelDiffGuardObject(row["annotation_styles"], `${at}.annotation_styles`)).map(([key0, value0]) => [key0, parseAnnotationStyleEntry(value0, `${at}.annotation_styles.${key0}`)])),
    families: row["families"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelDiffGuardObject(row["families"], `${at}.families`)).map(([key0, value0]) => [key0, parseFamilyEntry(value0, `${at}.families.${key0}`)])),
    family_parameters: row["family_parameters"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelDiffGuardObject(row["family_parameters"], `${at}.family_parameters`)).map(([key0, value0]) => [key0, parseFamilyParameterEntry(value0, `${at}.family_parameters.${key0}`)])),
    family_solids: row["family_solids"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelDiffGuardObject(row["family_solids"], `${at}.family_solids`)).map(([key0, value0]) => [key0, parseFamilySolidEntry(value0, `${at}.family_solids.${key0}`)])),
    components: row["components"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelDiffGuardObject(row["components"], `${at}.components`)).map(([key0, value0]) => [key0, parseComponentEntry(value0, `${at}.components.${key0}`)])),
    component_overrides: row["component_overrides"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelDiffGuardObject(row["component_overrides"], `${at}.component_overrides`)).map(([key0, value0]) => [key0, parseComponentOverrideEntry(value0, `${at}.component_overrides.${key0}`)])),
    mep_elements: row["mep_elements"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelDiffGuardObject(row["mep_elements"], `${at}.mep_elements`)).map(([key0, value0]) => [key0, parseMepElementEntry(value0, `${at}.mep_elements.${key0}`)])),
    clash_sets: row["clash_sets"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelDiffGuardObject(row["clash_sets"], `${at}.clash_sets`)).map(([key0, value0]) => [key0, parseClashSetEntry(value0, `${at}.clash_sets.${key0}`)])),
    rules: row["rules"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelDiffGuardObject(row["rules"], `${at}.rules`)).map(([key0, value0]) => [key0, parseRuleEntry(value0, `${at}.rules.${key0}`)])),
    issues: row["issues"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelDiffGuardObject(row["issues"], `${at}.issues`)).map(([key0, value0]) => [key0, parseIssueEntry(value0, `${at}.issues.${key0}`)])),
    issue_comments: row["issue_comments"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelDiffGuardObject(row["issue_comments"], `${at}.issue_comments`)).map(([key0, value0]) => [key0, parseIssueCommentEntry(value0, `${at}.issue_comments.${key0}`)])),
    wall_sweeps: row["wall_sweeps"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelDiffGuardObject(row["wall_sweeps"], `${at}.wall_sweeps`)).map(([key0, value0]) => [key0, parseWallSweepEntry(value0, `${at}.wall_sweeps.${key0}`)])),
    schedules: row["schedules"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelDiffGuardObject(row["schedules"], `${at}.schedules`)).map(([key0, value0]) => [key0, parseScheduleEntry(value0, `${at}.schedules.${key0}`)])),
    property_templates: row["property_templates"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelDiffGuardObject(row["property_templates"], `${at}.property_templates`)).map(([key0, value0]) => [key0, parsePropertyTemplateEntry(value0, `${at}.property_templates.${key0}`)])),
    classification_systems: row["classification_systems"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelDiffGuardObject(row["classification_systems"], `${at}.classification_systems`)).map(([key0, value0]) => [key0, parseClassificationSystemEntry(value0, `${at}.classification_systems.${key0}`)])),
    properties: row["properties"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelDiffGuardObject(row["properties"], `${at}.properties`)).map(([key0, value0]) => [key0, parsePropertySetEntry(value0, `${at}.properties.${key0}`)])),
    classifications: row["classifications"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelDiffGuardObject(row["classifications"], `${at}.classifications`)).map(([key0, value0]) => [key0, parseClassificationSetEntry(value0, `${at}.classifications.${key0}`)])),
    supports: row["supports"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelDiffGuardObject(row["supports"], `${at}.supports`)).map(([key0, value0]) => [key0, parseStructuralSupportEntry(value0, `${at}.supports.${key0}`)])),
    load_cases: row["load_cases"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelDiffGuardObject(row["load_cases"], `${at}.load_cases`)).map(([key0, value0]) => [key0, parseLoadCaseEntry(value0, `${at}.load_cases.${key0}`)])),
    loads: row["loads"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelDiffGuardObject(row["loads"], `${at}.loads`)).map(([key0, value0]) => [key0, parseStructuralLoadEntry(value0, `${at}.loads.${key0}`)])),
    option_groups: row["option_groups"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelDiffGuardObject(row["option_groups"], `${at}.option_groups`)).map(([key0, value0]) => [key0, parseOptionGroupEntry(value0, `${at}.option_groups.${key0}`)])),
    design_options: row["design_options"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelDiffGuardObject(row["design_options"], `${at}.design_options`)).map(([key0, value0]) => [key0, parseDesignOptionEntry(value0, `${at}.design_options.${key0}`)])),
    worksets: row["worksets"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelDiffGuardObject(row["worksets"], `${at}.worksets`)).map(([key0, value0]) => [key0, parseWorksetEntry(value0, `${at}.worksets.${key0}`)])),
    element_options: row["element_options"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelDiffGuardObject(row["element_options"], `${at}.element_options`)).map(([key0, value0]) => [key0, parseElementMembershipEntry(value0, `${at}.element_options.${key0}`)])),
    element_worksets: row["element_worksets"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelDiffGuardObject(row["element_worksets"], `${at}.element_worksets`)).map(([key0, value0]) => [key0, parseElementMembershipEntry(value0, `${at}.element_worksets.${key0}`)])),
  };
}

export function parseOpeningEntry(value: unknown, at = "$"): OpeningEntry {
  return ((row0) => { switch (bimModelDiffGuardMember(row0["entry"], `${at}.entry`, ["Created", "Deleted", "Replaced", "Patched"] as const)) { case "Created": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Created", value: parseOpening(row0["value"], `${at}.value`) }; case "Deleted": bimModelDiffGuardClosed(row0, `${at}`, ["entry"]); return { entry: "Deleted" }; case "Replaced": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Replaced", value: parseOpening(row0["value"], `${at}.value`) }; case "Patched": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Patched", value: parseOpeningPatch(row0["value"], `${at}.value`) }; } })(bimModelDiffGuardObject(value, `${at}`));
}

export function parseOpeningPatch(value: unknown, at = "$"): OpeningPatch {
  const row = bimModelDiffGuardClosed(bimModelDiffGuardObject(value, at), at, ["host", "kind", "offset", "sill_override", "width", "height", "flip_hand", "flip_facing", "name", "reveal_depth", "reveal_material"]);
  return {
    host: row["host"] === undefined ? undefined : bimModelDiffGuardString(row["host"], `${at}.host`),
    kind: row["kind"] === undefined ? undefined : parseOpeningKind(row["kind"], `${at}.kind`),
    offset: row["offset"] === undefined ? undefined : bimModelDiffGuardNumber(row["offset"], `${at}.offset`),
    sill_override: row["sill_override"] === undefined ? undefined : ((row0) => ({ value: (row0["value"] === null ? null : bimModelDiffGuardNumber(row0["value"], `${at}.sill_override.value`)) }))(bimModelDiffGuardClosed(bimModelDiffGuardObject(row["sill_override"], `${at}.sill_override`), `${at}.sill_override`, ["value"])),
    width: row["width"] === undefined ? undefined : ((row0) => ({ value: (row0["value"] === null ? null : bimModelDiffGuardNumber(row0["value"], `${at}.width.value`)) }))(bimModelDiffGuardClosed(bimModelDiffGuardObject(row["width"], `${at}.width`), `${at}.width`, ["value"])),
    height: row["height"] === undefined ? undefined : ((row0) => ({ value: (row0["value"] === null ? null : bimModelDiffGuardNumber(row0["value"], `${at}.height.value`)) }))(bimModelDiffGuardClosed(bimModelDiffGuardObject(row["height"], `${at}.height`), `${at}.height`, ["value"])),
    flip_hand: row["flip_hand"] === undefined ? undefined : bimModelDiffGuardBoolean(row["flip_hand"], `${at}.flip_hand`),
    flip_facing: row["flip_facing"] === undefined ? undefined : bimModelDiffGuardBoolean(row["flip_facing"], `${at}.flip_facing`),
    name: row["name"] === undefined ? undefined : bimModelDiffGuardString(row["name"], `${at}.name`),
    reveal_depth: row["reveal_depth"] === undefined ? undefined : ((row0) => ({ value: (row0["value"] === null ? null : bimModelDiffGuardNumber(row0["value"], `${at}.reveal_depth.value`)) }))(bimModelDiffGuardClosed(bimModelDiffGuardObject(row["reveal_depth"], `${at}.reveal_depth`), `${at}.reveal_depth`, ["value"])),
    reveal_material: row["reveal_material"] === undefined ? undefined : ((row0) => ({ value: (row0["value"] === null ? null : bimModelDiffGuardString(row0["value"], `${at}.reveal_material.value`)) }))(bimModelDiffGuardClosed(bimModelDiffGuardObject(row["reveal_material"], `${at}.reveal_material`), `${at}.reveal_material`, ["value"])),
  };
}

export function parseProjectPatch(value: unknown, at = "$"): ProjectPatch {
  const row = bimModelDiffGuardClosed(bimModelDiffGuardObject(value, at), at, ["name", "description", "author", "organization", "phase_names"]);
  return {
    name: row["name"] === undefined ? undefined : bimModelDiffGuardString(row["name"], `${at}.name`),
    description: row["description"] === undefined ? undefined : bimModelDiffGuardString(row["description"], `${at}.description`),
    author: row["author"] === undefined ? undefined : bimModelDiffGuardString(row["author"], `${at}.author`),
    organization: row["organization"] === undefined ? undefined : bimModelDiffGuardString(row["organization"], `${at}.organization`),
    phase_names: row["phase_names"] === undefined ? undefined : bimModelDiffGuardArray(row["phase_names"], `${at}.phase_names`).map((item, index) => bimModelDiffGuardString(item, `${at}.phase_names[${index}]`)),
  };
}

export function parsePropertySetEntry(value: unknown, at = "$"): PropertySetEntry {
  return ((row0) => { switch (bimModelDiffGuardMember(row0["entry"], `${at}.entry`, ["Created", "Deleted", "Replaced", "Patched"] as const)) { case "Created": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Created", value: parsePropertySet(row0["value"], `${at}.value`) }; case "Deleted": bimModelDiffGuardClosed(row0, `${at}`, ["entry"]); return { entry: "Deleted" }; case "Replaced": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Replaced", value: parsePropertySet(row0["value"], `${at}.value`) }; case "Patched": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Patched", value: parsePropertySetPatch(row0["value"], `${at}.value`) }; } })(bimModelDiffGuardObject(value, `${at}`));
}

export function parsePropertySetPatch(value: unknown, at = "$"): PropertySetPatch {
  const row = bimModelDiffGuardClosed(bimModelDiffGuardObject(value, at), at, ["assigned"]);
  return {
    assigned: row["assigned"] === undefined ? undefined : Object.fromEntries(Object.entries(bimModelDiffGuardObject(row["assigned"], `${at}.assigned`)).map(([key0, value0]) => [key0, Object.fromEntries(Object.entries(bimModelDiffGuardObject(value0, `${at}.assigned.${key0}`)).map(([key1, value1]) => [key1, (value1 === null ? null : parsePropertyValue(value1, `${at}.assigned.${key0}.${key1}`))]))])),
  };
}

export function parsePropertyTemplateEntry(value: unknown, at = "$"): PropertyTemplateEntry {
  return ((row0) => { switch (bimModelDiffGuardMember(row0["entry"], `${at}.entry`, ["Created", "Deleted", "Replaced", "Patched"] as const)) { case "Created": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Created", value: parsePropertyTemplate(row0["value"], `${at}.value`) }; case "Deleted": bimModelDiffGuardClosed(row0, `${at}`, ["entry"]); return { entry: "Deleted" }; case "Replaced": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Replaced", value: parsePropertyTemplate(row0["value"], `${at}.value`) }; case "Patched": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Patched", value: parsePropertyTemplatePatch(row0["value"], `${at}.value`) }; } })(bimModelDiffGuardObject(value, `${at}`));
}

export function parsePropertyTemplatePatch(value: unknown, at = "$"): PropertyTemplatePatch {
  const row = bimModelDiffGuardClosed(bimModelDiffGuardObject(value, at), at, ["name", "applies_to", "properties"]);
  return {
    name: row["name"] === undefined ? undefined : bimModelDiffGuardString(row["name"], `${at}.name`),
    applies_to: row["applies_to"] === undefined ? undefined : bimModelDiffGuardArray(row["applies_to"], `${at}.applies_to`).map((item, index) => parseTemplateTarget(item, `${at}.applies_to[${index}]`)),
    properties: row["properties"] === undefined ? undefined : bimModelDiffGuardArray(row["properties"], `${at}.properties`).map((item, index) => parsePropertyDef(item, `${at}.properties[${index}]`)),
  };
}

export function parseRailingEntry(value: unknown, at = "$"): RailingEntry {
  return ((row0) => { switch (bimModelDiffGuardMember(row0["entry"], `${at}.entry`, ["Created", "Deleted", "Replaced", "Patched"] as const)) { case "Created": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Created", value: parseRailing(row0["value"], `${at}.value`) }; case "Deleted": bimModelDiffGuardClosed(row0, `${at}`, ["entry"]); return { entry: "Deleted" }; case "Replaced": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Replaced", value: parseRailing(row0["value"], `${at}.value`) }; case "Patched": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Patched", value: parseRailingPatch(row0["value"], `${at}.value`) }; } })(bimModelDiffGuardObject(value, `${at}`));
}

export function parseRailingPatch(value: unknown, at = "$"): RailingPatch {
  const row = bimModelDiffGuardClosed(bimModelDiffGuardObject(value, at), at, ["storey", "path", "height", "post_spacing", "profile", "post_profile", "baluster", "infill", "material", "base_offset", "host", "phase", "name"]);
  return {
    storey: row["storey"] === undefined ? undefined : bimModelDiffGuardString(row["storey"], `${at}.storey`),
    path: row["path"] === undefined ? undefined : bimModelDiffGuardArray(row["path"], `${at}.path`).map((item, index) => parsePoint2(item, `${at}.path[${index}]`)),
    height: row["height"] === undefined ? undefined : bimModelDiffGuardNumber(row["height"], `${at}.height`),
    post_spacing: row["post_spacing"] === undefined ? undefined : bimModelDiffGuardNumber(row["post_spacing"], `${at}.post_spacing`),
    profile: row["profile"] === undefined ? undefined : parseProfile(row["profile"], `${at}.profile`),
    post_profile: row["post_profile"] === undefined ? undefined : parseProfile(row["post_profile"], `${at}.post_profile`),
    baluster: row["baluster"] === undefined ? undefined : ((row0) => ({ value: (row0["value"] === null ? null : parseBaluster(row0["value"], `${at}.baluster.value`)) }))(bimModelDiffGuardClosed(bimModelDiffGuardObject(row["baluster"], `${at}.baluster`), `${at}.baluster`, ["value"])),
    infill: row["infill"] === undefined ? undefined : parseInfill(row["infill"], `${at}.infill`),
    material: row["material"] === undefined ? undefined : bimModelDiffGuardString(row["material"], `${at}.material`),
    base_offset: row["base_offset"] === undefined ? undefined : bimModelDiffGuardNumber(row["base_offset"], `${at}.base_offset`),
    host: row["host"] === undefined ? undefined : ((row0) => ({ value: (row0["value"] === null ? null : parseRailingHost(row0["value"], `${at}.host.value`)) }))(bimModelDiffGuardClosed(bimModelDiffGuardObject(row["host"], `${at}.host`), `${at}.host`, ["value"])),
    phase: row["phase"] === undefined ? undefined : parsePhase(row["phase"], `${at}.phase`),
    name: row["name"] === undefined ? undefined : bimModelDiffGuardString(row["name"], `${at}.name`),
  };
}

export function parseRampEntry(value: unknown, at = "$"): RampEntry {
  return ((row0) => { switch (bimModelDiffGuardMember(row0["entry"], `${at}.entry`, ["Created", "Deleted", "Replaced", "Patched"] as const)) { case "Created": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Created", value: parseRamp(row0["value"], `${at}.value`) }; case "Deleted": bimModelDiffGuardClosed(row0, `${at}`, ["entry"]); return { entry: "Deleted" }; case "Replaced": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Replaced", value: parseRamp(row0["value"], `${at}.value`) }; case "Patched": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Patched", value: parseRampPatch(row0["value"], `${at}.value`) }; } })(bimModelDiffGuardObject(value, `${at}`));
}

export function parseRampPatch(value: unknown, at = "$"): RampPatch {
  const row = bimModelDiffGuardClosed(bimModelDiffGuardObject(value, at), at, ["storey", "path", "width", "landing_start", "landing_end", "landing_turn", "max_slope", "thickness", "material", "base_offset", "top", "railing_left", "railing_right", "name"]);
  return {
    storey: row["storey"] === undefined ? undefined : bimModelDiffGuardString(row["storey"], `${at}.storey`),
    path: row["path"] === undefined ? undefined : bimModelDiffGuardArray(row["path"], `${at}.path`).map((item, index) => parseVertex(item, `${at}.path[${index}]`)),
    width: row["width"] === undefined ? undefined : bimModelDiffGuardNumber(row["width"], `${at}.width`),
    landing_start: row["landing_start"] === undefined ? undefined : bimModelDiffGuardNumber(row["landing_start"], `${at}.landing_start`),
    landing_end: row["landing_end"] === undefined ? undefined : bimModelDiffGuardNumber(row["landing_end"], `${at}.landing_end`),
    landing_turn: row["landing_turn"] === undefined ? undefined : bimModelDiffGuardNumber(row["landing_turn"], `${at}.landing_turn`),
    max_slope: row["max_slope"] === undefined ? undefined : bimModelDiffGuardNumber(row["max_slope"], `${at}.max_slope`),
    thickness: row["thickness"] === undefined ? undefined : bimModelDiffGuardNumber(row["thickness"], `${at}.thickness`),
    material: row["material"] === undefined ? undefined : bimModelDiffGuardString(row["material"], `${at}.material`),
    base_offset: row["base_offset"] === undefined ? undefined : bimModelDiffGuardNumber(row["base_offset"], `${at}.base_offset`),
    top: row["top"] === undefined ? undefined : parseTopConstraint(row["top"], `${at}.top`),
    railing_left: row["railing_left"] === undefined ? undefined : bimModelDiffGuardBoolean(row["railing_left"], `${at}.railing_left`),
    railing_right: row["railing_right"] === undefined ? undefined : bimModelDiffGuardBoolean(row["railing_right"], `${at}.railing_right`),
    name: row["name"] === undefined ? undefined : bimModelDiffGuardString(row["name"], `${at}.name`),
  };
}

export function parseRoofEntry(value: unknown, at = "$"): RoofEntry {
  return ((row0) => { switch (bimModelDiffGuardMember(row0["entry"], `${at}.entry`, ["Created", "Deleted", "Replaced", "Patched"] as const)) { case "Created": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Created", value: parseRoof(row0["value"], `${at}.value`) }; case "Deleted": bimModelDiffGuardClosed(row0, `${at}`, ["entry"]); return { entry: "Deleted" }; case "Replaced": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Replaced", value: parseRoof(row0["value"], `${at}.value`) }; case "Patched": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Patched", value: parseRoofPatch(row0["value"], `${at}.value`) }; } })(bimModelDiffGuardObject(value, `${at}`));
}

export function parseRoofPatch(value: unknown, at = "$"): RoofPatch {
  const row = bimModelDiffGuardClosed(bimModelDiffGuardObject(value, at), at, ["storey", "roof_type", "footprint", "shape", "overhang", "base_offset", "phase", "name"]);
  return {
    storey: row["storey"] === undefined ? undefined : bimModelDiffGuardString(row["storey"], `${at}.storey`),
    roof_type: row["roof_type"] === undefined ? undefined : bimModelDiffGuardString(row["roof_type"], `${at}.roof_type`),
    footprint: row["footprint"] === undefined ? undefined : bimModelDiffGuardArray(row["footprint"], `${at}.footprint`).map((item, index) => parseVertex(item, `${at}.footprint[${index}]`)),
    shape: row["shape"] === undefined ? undefined : parseRoofShape(row["shape"], `${at}.shape`),
    overhang: row["overhang"] === undefined ? undefined : bimModelDiffGuardNumber(row["overhang"], `${at}.overhang`),
    base_offset: row["base_offset"] === undefined ? undefined : bimModelDiffGuardNumber(row["base_offset"], `${at}.base_offset`),
    phase: row["phase"] === undefined ? undefined : parsePhase(row["phase"], `${at}.phase`),
    name: row["name"] === undefined ? undefined : bimModelDiffGuardString(row["name"], `${at}.name`),
  };
}

export function parseRoofTypeEntry(value: unknown, at = "$"): RoofTypeEntry {
  return ((row0) => { switch (bimModelDiffGuardMember(row0["entry"], `${at}.entry`, ["Created", "Deleted", "Replaced", "Patched"] as const)) { case "Created": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Created", value: parseRoofType(row0["value"], `${at}.value`) }; case "Deleted": bimModelDiffGuardClosed(row0, `${at}`, ["entry"]); return { entry: "Deleted" }; case "Replaced": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Replaced", value: parseRoofType(row0["value"], `${at}.value`) }; case "Patched": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Patched", value: parseRoofTypePatch(row0["value"], `${at}.value`) }; } })(bimModelDiffGuardObject(value, `${at}`));
}

export function parseRoofTypePatch(value: unknown, at = "$"): RoofTypePatch {
  const row = bimModelDiffGuardClosed(bimModelDiffGuardObject(value, at), at, ["name", "layers"]);
  return {
    name: row["name"] === undefined ? undefined : bimModelDiffGuardString(row["name"], `${at}.name`),
    layers: row["layers"] === undefined ? undefined : bimModelDiffGuardArray(row["layers"], `${at}.layers`).map((item, index) => parseLayer(item, `${at}.layers[${index}]`)),
  };
}

export function parseRuleEntry(value: unknown, at = "$"): RuleEntry {
  return ((row0) => { switch (bimModelDiffGuardMember(row0["entry"], `${at}.entry`, ["Created", "Deleted", "Replaced", "Patched"] as const)) { case "Created": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Created", value: parseRule(row0["value"], `${at}.value`) }; case "Deleted": bimModelDiffGuardClosed(row0, `${at}`, ["entry"]); return { entry: "Deleted" }; case "Replaced": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Replaced", value: parseRule(row0["value"], `${at}.value`) }; case "Patched": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Patched", value: parseRulePatch(row0["value"], `${at}.value`) }; } })(bimModelDiffGuardObject(value, `${at}`));
}

export function parseRulePatch(value: unknown, at = "$"): RulePatch {
  const row = bimModelDiffGuardClosed(bimModelDiffGuardObject(value, at), at, ["name", "kind", "limit", "severity", "scope"]);
  return {
    name: row["name"] === undefined ? undefined : bimModelDiffGuardString(row["name"], `${at}.name`),
    kind: row["kind"] === undefined ? undefined : parseRuleKind(row["kind"], `${at}.kind`),
    limit: row["limit"] === undefined ? undefined : bimModelDiffGuardNumber(row["limit"], `${at}.limit`),
    severity: row["severity"] === undefined ? undefined : parseRuleSeverity(row["severity"], `${at}.severity`),
    scope: row["scope"] === undefined ? undefined : parseRuleScope(row["scope"], `${at}.scope`),
  };
}

export function parseScheduleEntry(value: unknown, at = "$"): ScheduleEntry {
  return ((row0) => { switch (bimModelDiffGuardMember(row0["entry"], `${at}.entry`, ["Created", "Deleted", "Replaced", "Patched"] as const)) { case "Created": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Created", value: parseSchedule(row0["value"], `${at}.value`) }; case "Deleted": bimModelDiffGuardClosed(row0, `${at}`, ["entry"]); return { entry: "Deleted" }; case "Replaced": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Replaced", value: parseSchedule(row0["value"], `${at}.value`) }; case "Patched": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Patched", value: parseSchedulePatch(row0["value"], `${at}.value`) }; } })(bimModelDiffGuardObject(value, `${at}`));
}

export function parseSchedulePatch(value: unknown, at = "$"): SchedulePatch {
  const row = bimModelDiffGuardClosed(bimModelDiffGuardObject(value, at), at, ["name", "category", "columns", "sort", "filter", "group", "itemize", "storeys", "phases"]);
  return {
    name: row["name"] === undefined ? undefined : bimModelDiffGuardString(row["name"], `${at}.name`),
    category: row["category"] === undefined ? undefined : parseScheduleCategory(row["category"], `${at}.category`),
    columns: row["columns"] === undefined ? undefined : bimModelDiffGuardArray(row["columns"], `${at}.columns`).map((item, index) => parseScheduleColumn(item, `${at}.columns[${index}]`)),
    sort: row["sort"] === undefined ? undefined : bimModelDiffGuardArray(row["sort"], `${at}.sort`).map((item, index) => parseScheduleSort(item, `${at}.sort[${index}]`)),
    filter: row["filter"] === undefined ? undefined : bimModelDiffGuardArray(row["filter"], `${at}.filter`).map((item, index) => parseScheduleFilter(item, `${at}.filter[${index}]`)),
    group: row["group"] === undefined ? undefined : bimModelDiffGuardArray(row["group"], `${at}.group`).map((item, index) => parseScheduleGroup(item, `${at}.group[${index}]`)),
    itemize: row["itemize"] === undefined ? undefined : bimModelDiffGuardBoolean(row["itemize"], `${at}.itemize`),
    storeys: row["storeys"] === undefined ? undefined : bimModelDiffGuardArray(row["storeys"], `${at}.storeys`).map((item, index) => bimModelDiffGuardString(item, `${at}.storeys[${index}]`)),
    phases: row["phases"] === undefined ? undefined : bimModelDiffGuardArray(row["phases"], `${at}.phases`).map((item, index) => parsePhase(item, `${at}.phases[${index}]`)),
  };
}

export function parseSheetEntry(value: unknown, at = "$"): SheetEntry {
  return ((row0) => { switch (bimModelDiffGuardMember(row0["entry"], `${at}.entry`, ["Created", "Deleted", "Replaced", "Patched"] as const)) { case "Created": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Created", value: parseSheet(row0["value"], `${at}.value`) }; case "Deleted": bimModelDiffGuardClosed(row0, `${at}`, ["entry"]); return { entry: "Deleted" }; case "Replaced": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Replaced", value: parseSheet(row0["value"], `${at}.value`) }; case "Patched": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Patched", value: parseSheetPatch(row0["value"], `${at}.value`) }; } })(bimModelDiffGuardObject(value, `${at}`));
}

export function parseSheetPatch(value: unknown, at = "$"): SheetPatch {
  const row = bimModelDiffGuardClosed(bimModelDiffGuardObject(value, at), at, ["number", "name", "paper", "orientation", "project", "drawn_by", "checked_by", "date", "revision", "scale_label"]);
  return {
    number: row["number"] === undefined ? undefined : bimModelDiffGuardString(row["number"], `${at}.number`),
    name: row["name"] === undefined ? undefined : bimModelDiffGuardString(row["name"], `${at}.name`),
    paper: row["paper"] === undefined ? undefined : parsePaper(row["paper"], `${at}.paper`),
    orientation: row["orientation"] === undefined ? undefined : parseOrientation(row["orientation"], `${at}.orientation`),
    project: row["project"] === undefined ? undefined : bimModelDiffGuardString(row["project"], `${at}.project`),
    drawn_by: row["drawn_by"] === undefined ? undefined : bimModelDiffGuardString(row["drawn_by"], `${at}.drawn_by`),
    checked_by: row["checked_by"] === undefined ? undefined : bimModelDiffGuardString(row["checked_by"], `${at}.checked_by`),
    date: row["date"] === undefined ? undefined : bimModelDiffGuardString(row["date"], `${at}.date`),
    revision: row["revision"] === undefined ? undefined : bimModelDiffGuardString(row["revision"], `${at}.revision`),
    scale_label: row["scale_label"] === undefined ? undefined : bimModelDiffGuardString(row["scale_label"], `${at}.scale_label`),
  };
}

export function parseSheetRevisionEntry(value: unknown, at = "$"): SheetRevisionEntry {
  return ((row0) => { switch (bimModelDiffGuardMember(row0["entry"], `${at}.entry`, ["Created", "Deleted", "Replaced", "Patched"] as const)) { case "Created": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Created", value: parseSheetRevision(row0["value"], `${at}.value`) }; case "Deleted": bimModelDiffGuardClosed(row0, `${at}`, ["entry"]); return { entry: "Deleted" }; case "Replaced": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Replaced", value: parseSheetRevision(row0["value"], `${at}.value`) }; case "Patched": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Patched", value: parseSheetRevisionPatch(row0["value"], `${at}.value`) }; } })(bimModelDiffGuardObject(value, `${at}`));
}

export function parseSheetRevisionPatch(value: unknown, at = "$"): SheetRevisionPatch {
  const row = bimModelDiffGuardClosed(bimModelDiffGuardObject(value, at), at, ["sheet", "number", "date", "description", "author"]);
  return {
    sheet: row["sheet"] === undefined ? undefined : bimModelDiffGuardString(row["sheet"], `${at}.sheet`),
    number: row["number"] === undefined ? undefined : bimModelDiffGuardString(row["number"], `${at}.number`),
    date: row["date"] === undefined ? undefined : bimModelDiffGuardString(row["date"], `${at}.date`),
    description: row["description"] === undefined ? undefined : bimModelDiffGuardString(row["description"], `${at}.description`),
    author: row["author"] === undefined ? undefined : bimModelDiffGuardString(row["author"], `${at}.author`),
  };
}

export function parseSiteEntry(value: unknown, at = "$"): SiteEntry {
  return ((row0) => { switch (bimModelDiffGuardMember(row0["entry"], `${at}.entry`, ["Created", "Deleted", "Replaced", "Patched"] as const)) { case "Created": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Created", value: parseSite(row0["value"], `${at}.value`) }; case "Deleted": bimModelDiffGuardClosed(row0, `${at}`, ["entry"]); return { entry: "Deleted" }; case "Replaced": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Replaced", value: parseSite(row0["value"], `${at}.value`) }; case "Patched": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Patched", value: parseSitePatch(row0["value"], `${at}.value`) }; } })(bimModelDiffGuardObject(value, `${at}`));
}

export function parseSitePatch(value: unknown, at = "$"): SitePatch {
  const row = bimModelDiffGuardClosed(bimModelDiffGuardObject(value, at), at, ["name", "latitude", "longitude", "elevation", "true_north", "boundary"]);
  return {
    name: row["name"] === undefined ? undefined : bimModelDiffGuardString(row["name"], `${at}.name`),
    latitude: row["latitude"] === undefined ? undefined : bimModelDiffGuardNumber(row["latitude"], `${at}.latitude`),
    longitude: row["longitude"] === undefined ? undefined : bimModelDiffGuardNumber(row["longitude"], `${at}.longitude`),
    elevation: row["elevation"] === undefined ? undefined : bimModelDiffGuardNumber(row["elevation"], `${at}.elevation`),
    true_north: row["true_north"] === undefined ? undefined : bimModelDiffGuardNumber(row["true_north"], `${at}.true_north`),
    boundary: row["boundary"] === undefined ? undefined : bimModelDiffGuardArray(row["boundary"], `${at}.boundary`).map((item, index) => parsePoint2(item, `${at}.boundary[${index}]`)),
  };
}

export function parseSlabEntry(value: unknown, at = "$"): SlabEntry {
  return ((row0) => { switch (bimModelDiffGuardMember(row0["entry"], `${at}.entry`, ["Created", "Deleted", "Replaced", "Patched"] as const)) { case "Created": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Created", value: parseSlab(row0["value"], `${at}.value`) }; case "Deleted": bimModelDiffGuardClosed(row0, `${at}`, ["entry"]); return { entry: "Deleted" }; case "Replaced": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Replaced", value: parseSlab(row0["value"], `${at}.value`) }; case "Patched": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Patched", value: parseSlabPatch(row0["value"], `${at}.value`) }; } })(bimModelDiffGuardObject(value, `${at}`));
}

export function parseSlabPatch(value: unknown, at = "$"): SlabPatch {
  const row = bimModelDiffGuardClosed(bimModelDiffGuardObject(value, at), at, ["storey", "slab_type", "boundary", "holes", "offset", "slope", "phase", "name"]);
  return {
    storey: row["storey"] === undefined ? undefined : bimModelDiffGuardString(row["storey"], `${at}.storey`),
    slab_type: row["slab_type"] === undefined ? undefined : bimModelDiffGuardString(row["slab_type"], `${at}.slab_type`),
    boundary: row["boundary"] === undefined ? undefined : bimModelDiffGuardArray(row["boundary"], `${at}.boundary`).map((item, index) => parseVertex(item, `${at}.boundary[${index}]`)),
    holes: row["holes"] === undefined ? undefined : bimModelDiffGuardArray(row["holes"], `${at}.holes`).map((item, index) => bimModelDiffGuardArray(item, `${at}.holes[${index}]`).map((item1, index1) => parseVertex(item1, `${at}.holes[${index}][${index1}]`))),
    offset: row["offset"] === undefined ? undefined : bimModelDiffGuardNumber(row["offset"], `${at}.offset`),
    slope: row["slope"] === undefined ? undefined : ((row0) => ({ value: (row0["value"] === null ? null : parseSlope(row0["value"], `${at}.slope.value`)) }))(bimModelDiffGuardClosed(bimModelDiffGuardObject(row["slope"], `${at}.slope`), `${at}.slope`, ["value"])),
    phase: row["phase"] === undefined ? undefined : parsePhase(row["phase"], `${at}.phase`),
    name: row["name"] === undefined ? undefined : bimModelDiffGuardString(row["name"], `${at}.name`),
  };
}

export function parseSlabTypeEntry(value: unknown, at = "$"): SlabTypeEntry {
  return ((row0) => { switch (bimModelDiffGuardMember(row0["entry"], `${at}.entry`, ["Created", "Deleted", "Replaced", "Patched"] as const)) { case "Created": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Created", value: parseSlabType(row0["value"], `${at}.value`) }; case "Deleted": bimModelDiffGuardClosed(row0, `${at}`, ["entry"]); return { entry: "Deleted" }; case "Replaced": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Replaced", value: parseSlabType(row0["value"], `${at}.value`) }; case "Patched": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Patched", value: parseSlabTypePatch(row0["value"], `${at}.value`) }; } })(bimModelDiffGuardObject(value, `${at}`));
}

export function parseSlabTypePatch(value: unknown, at = "$"): SlabTypePatch {
  const row = bimModelDiffGuardClosed(bimModelDiffGuardObject(value, at), at, ["name", "layers"]);
  return {
    name: row["name"] === undefined ? undefined : bimModelDiffGuardString(row["name"], `${at}.name`),
    layers: row["layers"] === undefined ? undefined : bimModelDiffGuardArray(row["layers"], `${at}.layers`).map((item, index) => parseLayer(item, `${at}.layers[${index}]`)),
  };
}

export function parseSpaceConditionsEntry(value: unknown, at = "$"): SpaceConditionsEntry {
  return ((row0) => { switch (bimModelDiffGuardMember(row0["entry"], `${at}.entry`, ["Created", "Deleted", "Replaced", "Patched"] as const)) { case "Created": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Created", value: parseSpaceConditions(row0["value"], `${at}.value`) }; case "Deleted": bimModelDiffGuardClosed(row0, `${at}`, ["entry"]); return { entry: "Deleted" }; case "Replaced": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Replaced", value: parseSpaceConditions(row0["value"], `${at}.value`) }; case "Patched": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Patched", value: parseSpaceConditionsPatch(row0["value"], `${at}.value`) }; } })(bimModelDiffGuardObject(value, `${at}`));
}

export function parseSpaceConditionsPatch(value: unknown, at = "$"): SpaceConditionsPatch {
  const row = bimModelDiffGuardClosed(bimModelDiffGuardObject(value, at), at, ["occupancy", "occupancy_density", "heating_setpoint", "cooling_setpoint", "ventilation_rate", "lighting_power_density", "equipment_power_density", "schedule"]);
  return {
    occupancy: row["occupancy"] === undefined ? undefined : ((row0) => ({ value: (row0["value"] === null ? null : bimModelDiffGuardString(row0["value"], `${at}.occupancy.value`)) }))(bimModelDiffGuardClosed(bimModelDiffGuardObject(row["occupancy"], `${at}.occupancy`), `${at}.occupancy`, ["value"])),
    occupancy_density: row["occupancy_density"] === undefined ? undefined : ((row0) => ({ value: (row0["value"] === null ? null : bimModelDiffGuardNumber(row0["value"], `${at}.occupancy_density.value`)) }))(bimModelDiffGuardClosed(bimModelDiffGuardObject(row["occupancy_density"], `${at}.occupancy_density`), `${at}.occupancy_density`, ["value"])),
    heating_setpoint: row["heating_setpoint"] === undefined ? undefined : ((row0) => ({ value: (row0["value"] === null ? null : bimModelDiffGuardNumber(row0["value"], `${at}.heating_setpoint.value`)) }))(bimModelDiffGuardClosed(bimModelDiffGuardObject(row["heating_setpoint"], `${at}.heating_setpoint`), `${at}.heating_setpoint`, ["value"])),
    cooling_setpoint: row["cooling_setpoint"] === undefined ? undefined : ((row0) => ({ value: (row0["value"] === null ? null : bimModelDiffGuardNumber(row0["value"], `${at}.cooling_setpoint.value`)) }))(bimModelDiffGuardClosed(bimModelDiffGuardObject(row["cooling_setpoint"], `${at}.cooling_setpoint`), `${at}.cooling_setpoint`, ["value"])),
    ventilation_rate: row["ventilation_rate"] === undefined ? undefined : ((row0) => ({ value: (row0["value"] === null ? null : bimModelDiffGuardNumber(row0["value"], `${at}.ventilation_rate.value`)) }))(bimModelDiffGuardClosed(bimModelDiffGuardObject(row["ventilation_rate"], `${at}.ventilation_rate`), `${at}.ventilation_rate`, ["value"])),
    lighting_power_density: row["lighting_power_density"] === undefined ? undefined : ((row0) => ({ value: (row0["value"] === null ? null : bimModelDiffGuardNumber(row0["value"], `${at}.lighting_power_density.value`)) }))(bimModelDiffGuardClosed(bimModelDiffGuardObject(row["lighting_power_density"], `${at}.lighting_power_density`), `${at}.lighting_power_density`, ["value"])),
    equipment_power_density: row["equipment_power_density"] === undefined ? undefined : ((row0) => ({ value: (row0["value"] === null ? null : bimModelDiffGuardNumber(row0["value"], `${at}.equipment_power_density.value`)) }))(bimModelDiffGuardClosed(bimModelDiffGuardObject(row["equipment_power_density"], `${at}.equipment_power_density`), `${at}.equipment_power_density`, ["value"])),
    schedule: row["schedule"] === undefined ? undefined : ((row0) => ({ value: (row0["value"] === null ? null : bimModelDiffGuardString(row0["value"], `${at}.schedule.value`)) }))(bimModelDiffGuardClosed(bimModelDiffGuardObject(row["schedule"], `${at}.schedule`), `${at}.schedule`, ["value"])),
  };
}

export function parseSpaceEntry(value: unknown, at = "$"): SpaceEntry {
  return ((row0) => { switch (bimModelDiffGuardMember(row0["entry"], `${at}.entry`, ["Created", "Deleted", "Replaced", "Patched"] as const)) { case "Created": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Created", value: parseSpace(row0["value"], `${at}.value`) }; case "Deleted": bimModelDiffGuardClosed(row0, `${at}`, ["entry"]); return { entry: "Deleted" }; case "Replaced": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Replaced", value: parseSpace(row0["value"], `${at}.value`) }; case "Patched": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Patched", value: parseSpacePatch(row0["value"], `${at}.value`) }; } })(bimModelDiffGuardObject(value, `${at}`));
}

export function parseSpacePatch(value: unknown, at = "$"): SpacePatch {
  const row = bimModelDiffGuardClosed(bimModelDiffGuardObject(value, at), at, ["storey", "number", "name", "boundary", "usage", "phase", "zone", "floor_finish", "wall_finish", "ceiling_finish"]);
  return {
    storey: row["storey"] === undefined ? undefined : bimModelDiffGuardString(row["storey"], `${at}.storey`),
    number: row["number"] === undefined ? undefined : bimModelDiffGuardString(row["number"], `${at}.number`),
    name: row["name"] === undefined ? undefined : bimModelDiffGuardString(row["name"], `${at}.name`),
    boundary: row["boundary"] === undefined ? undefined : parseSpaceBoundary(row["boundary"], `${at}.boundary`),
    usage: row["usage"] === undefined ? undefined : bimModelDiffGuardString(row["usage"], `${at}.usage`),
    phase: row["phase"] === undefined ? undefined : parsePhase(row["phase"], `${at}.phase`),
    zone: row["zone"] === undefined ? undefined : ((row0) => ({ value: (row0["value"] === null ? null : bimModelDiffGuardString(row0["value"], `${at}.zone.value`)) }))(bimModelDiffGuardClosed(bimModelDiffGuardObject(row["zone"], `${at}.zone`), `${at}.zone`, ["value"])),
    floor_finish: row["floor_finish"] === undefined ? undefined : ((row0) => ({ value: (row0["value"] === null ? null : bimModelDiffGuardString(row0["value"], `${at}.floor_finish.value`)) }))(bimModelDiffGuardClosed(bimModelDiffGuardObject(row["floor_finish"], `${at}.floor_finish`), `${at}.floor_finish`, ["value"])),
    wall_finish: row["wall_finish"] === undefined ? undefined : ((row0) => ({ value: (row0["value"] === null ? null : bimModelDiffGuardString(row0["value"], `${at}.wall_finish.value`)) }))(bimModelDiffGuardClosed(bimModelDiffGuardObject(row["wall_finish"], `${at}.wall_finish`), `${at}.wall_finish`, ["value"])),
    ceiling_finish: row["ceiling_finish"] === undefined ? undefined : ((row0) => ({ value: (row0["value"] === null ? null : bimModelDiffGuardString(row0["value"], `${at}.ceiling_finish.value`)) }))(bimModelDiffGuardClosed(bimModelDiffGuardObject(row["ceiling_finish"], `${at}.ceiling_finish`), `${at}.ceiling_finish`, ["value"])),
  };
}

export function parseStairEntry(value: unknown, at = "$"): StairEntry {
  return ((row0) => { switch (bimModelDiffGuardMember(row0["entry"], `${at}.entry`, ["Created", "Deleted", "Replaced", "Patched"] as const)) { case "Created": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Created", value: parseStair(row0["value"], `${at}.value`) }; case "Deleted": bimModelDiffGuardClosed(row0, `${at}`, ["entry"]); return { entry: "Deleted" }; case "Replaced": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Replaced", value: parseStair(row0["value"], `${at}.value`) }; case "Patched": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Patched", value: parseStairPatch(row0["value"], `${at}.value`) }; } })(bimModelDiffGuardObject(value, `${at}`));
}

export function parseStairPatch(value: unknown, at = "$"): StairPatch {
  const row = bimModelDiffGuardClosed(bimModelDiffGuardObject(value, at), at, ["storey", "start", "direction", "width", "flight", "top", "max_riser", "min_tread", "stringer", "nosing", "tread_thickness", "riser", "landing_depth", "phase", "name"]);
  return {
    storey: row["storey"] === undefined ? undefined : bimModelDiffGuardString(row["storey"], `${at}.storey`),
    start: row["start"] === undefined ? undefined : parsePoint2(row["start"], `${at}.start`),
    direction: row["direction"] === undefined ? undefined : bimModelDiffGuardNumber(row["direction"], `${at}.direction`),
    width: row["width"] === undefined ? undefined : bimModelDiffGuardNumber(row["width"], `${at}.width`),
    flight: row["flight"] === undefined ? undefined : parseStairFlight(row["flight"], `${at}.flight`),
    top: row["top"] === undefined ? undefined : parseTopConstraint(row["top"], `${at}.top`),
    max_riser: row["max_riser"] === undefined ? undefined : bimModelDiffGuardNumber(row["max_riser"], `${at}.max_riser`),
    min_tread: row["min_tread"] === undefined ? undefined : bimModelDiffGuardNumber(row["min_tread"], `${at}.min_tread`),
    stringer: row["stringer"] === undefined ? undefined : parseStairStringer(row["stringer"], `${at}.stringer`),
    nosing: row["nosing"] === undefined ? undefined : bimModelDiffGuardNumber(row["nosing"], `${at}.nosing`),
    tread_thickness: row["tread_thickness"] === undefined ? undefined : bimModelDiffGuardNumber(row["tread_thickness"], `${at}.tread_thickness`),
    riser: row["riser"] === undefined ? undefined : parseRiserKind(row["riser"], `${at}.riser`),
    landing_depth: row["landing_depth"] === undefined ? undefined : bimModelDiffGuardNumber(row["landing_depth"], `${at}.landing_depth`),
    phase: row["phase"] === undefined ? undefined : parsePhase(row["phase"], `${at}.phase`),
    name: row["name"] === undefined ? undefined : bimModelDiffGuardString(row["name"], `${at}.name`),
  };
}

export function parseStoreyEntry(value: unknown, at = "$"): StoreyEntry {
  return ((row0) => { switch (bimModelDiffGuardMember(row0["entry"], `${at}.entry`, ["Created", "Deleted", "Replaced", "Patched"] as const)) { case "Created": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Created", value: parseStorey(row0["value"], `${at}.value`) }; case "Deleted": bimModelDiffGuardClosed(row0, `${at}`, ["entry"]); return { entry: "Deleted" }; case "Replaced": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Replaced", value: parseStorey(row0["value"], `${at}.value`) }; case "Patched": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Patched", value: parseStoreyPatch(row0["value"], `${at}.value`) }; } })(bimModelDiffGuardObject(value, `${at}`));
}

export function parseStoreyPatch(value: unknown, at = "$"): StoreyPatch {
  const row = bimModelDiffGuardClosed(bimModelDiffGuardObject(value, at), at, ["building", "name", "level", "height", "cut_height"]);
  return {
    building: row["building"] === undefined ? undefined : bimModelDiffGuardString(row["building"], `${at}.building`),
    name: row["name"] === undefined ? undefined : bimModelDiffGuardString(row["name"], `${at}.name`),
    level: row["level"] === undefined ? undefined : bimModelDiffGuardInteger(row["level"], `${at}.level`),
    height: row["height"] === undefined ? undefined : bimModelDiffGuardNumber(row["height"], `${at}.height`),
    cut_height: row["cut_height"] === undefined ? undefined : ((row0) => ({ value: (row0["value"] === null ? null : bimModelDiffGuardNumber(row0["value"], `${at}.cut_height.value`)) }))(bimModelDiffGuardClosed(bimModelDiffGuardObject(row["cut_height"], `${at}.cut_height`), `${at}.cut_height`, ["value"])),
  };
}

export function parseStructuralLoadEntry(value: unknown, at = "$"): StructuralLoadEntry {
  return ((row0) => { switch (bimModelDiffGuardMember(row0["entry"], `${at}.entry`, ["Created", "Replaced", "Deleted", "Patched"] as const)) { case "Created": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Created", value: ((row1) => ({ name: bimModelDiffGuardString(row1["name"], `${at}.value.name`), load_case: bimModelDiffGuardString(row1["load_case"], `${at}.value.load_case`), member: bimModelDiffGuardString(row1["member"], `${at}.value.member`), location: parseStructuralLocation(row1["location"], `${at}.value.location`), force: parsePoint3(row1["force"], `${at}.value.force`), moment: parsePoint3(row1["moment"], `${at}.value.moment`) }))(bimModelDiffGuardClosed(bimModelDiffGuardObject(row0["value"], `${at}.value`), `${at}.value`, ["name", "load_case", "member", "location", "force", "moment"])) }; case "Replaced": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Replaced", value: ((row1) => ({ name: bimModelDiffGuardString(row1["name"], `${at}.value.name`), load_case: bimModelDiffGuardString(row1["load_case"], `${at}.value.load_case`), member: bimModelDiffGuardString(row1["member"], `${at}.value.member`), location: parseStructuralLocation(row1["location"], `${at}.value.location`), force: parsePoint3(row1["force"], `${at}.value.force`), moment: parsePoint3(row1["moment"], `${at}.value.moment`) }))(bimModelDiffGuardClosed(bimModelDiffGuardObject(row0["value"], `${at}.value`), `${at}.value`, ["name", "load_case", "member", "location", "force", "moment"])) }; case "Deleted": bimModelDiffGuardClosed(row0, `${at}`, ["entry"]); return { entry: "Deleted" }; case "Patched": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Patched", value: ((row1) => ({ name: row1["name"] === undefined ? undefined : bimModelDiffGuardString(row1["name"], `${at}.value.name`), load_case: row1["load_case"] === undefined ? undefined : bimModelDiffGuardString(row1["load_case"], `${at}.value.load_case`), member: row1["member"] === undefined ? undefined : bimModelDiffGuardString(row1["member"], `${at}.value.member`), location: row1["location"] === undefined ? undefined : parseStructuralLocation(row1["location"], `${at}.value.location`), force: row1["force"] === undefined ? undefined : parsePoint3(row1["force"], `${at}.value.force`), moment: row1["moment"] === undefined ? undefined : parsePoint3(row1["moment"], `${at}.value.moment`) }))(bimModelDiffGuardClosed(bimModelDiffGuardObject(row0["value"], `${at}.value`), `${at}.value`, ["name", "load_case", "member", "location", "force", "moment"])) }; } })(bimModelDiffGuardObject(value, `${at}`));
}

export function parseStructuralLoadPatch(value: unknown, at = "$"): StructuralLoadPatch {
  const row = bimModelDiffGuardClosed(bimModelDiffGuardObject(value, at), at, ["name", "load_case", "member", "location", "force", "moment"]);
  return {
    name: row["name"] === undefined ? undefined : bimModelDiffGuardString(row["name"], `${at}.name`),
    load_case: row["load_case"] === undefined ? undefined : bimModelDiffGuardString(row["load_case"], `${at}.load_case`),
    member: row["member"] === undefined ? undefined : bimModelDiffGuardString(row["member"], `${at}.member`),
    location: row["location"] === undefined ? undefined : parseStructuralLocation(row["location"], `${at}.location`),
    force: row["force"] === undefined ? undefined : parsePoint3(row["force"], `${at}.force`),
    moment: row["moment"] === undefined ? undefined : parsePoint3(row["moment"], `${at}.moment`),
  };
}

export function parseStructuralSupportEntry(value: unknown, at = "$"): StructuralSupportEntry {
  return ((row0) => { switch (bimModelDiffGuardMember(row0["entry"], `${at}.entry`, ["Created", "Replaced", "Deleted", "Patched"] as const)) { case "Created": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Created", value: ((row1) => ({ name: bimModelDiffGuardString(row1["name"], `${at}.value.name`), member: bimModelDiffGuardString(row1["member"], `${at}.value.member`), location: parseStructuralLocation(row1["location"], `${at}.value.location`), offset: parsePoint3(row1["offset"], `${at}.value.offset`), restraints: parseRestraints(row1["restraints"], `${at}.value.restraints`) }))(bimModelDiffGuardClosed(bimModelDiffGuardObject(row0["value"], `${at}.value`), `${at}.value`, ["name", "member", "location", "offset", "restraints"])) }; case "Replaced": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Replaced", value: ((row1) => ({ name: bimModelDiffGuardString(row1["name"], `${at}.value.name`), member: bimModelDiffGuardString(row1["member"], `${at}.value.member`), location: parseStructuralLocation(row1["location"], `${at}.value.location`), offset: parsePoint3(row1["offset"], `${at}.value.offset`), restraints: parseRestraints(row1["restraints"], `${at}.value.restraints`) }))(bimModelDiffGuardClosed(bimModelDiffGuardObject(row0["value"], `${at}.value`), `${at}.value`, ["name", "member", "location", "offset", "restraints"])) }; case "Deleted": bimModelDiffGuardClosed(row0, `${at}`, ["entry"]); return { entry: "Deleted" }; case "Patched": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Patched", value: ((row1) => ({ name: row1["name"] === undefined ? undefined : bimModelDiffGuardString(row1["name"], `${at}.value.name`), member: row1["member"] === undefined ? undefined : bimModelDiffGuardString(row1["member"], `${at}.value.member`), location: row1["location"] === undefined ? undefined : parseStructuralLocation(row1["location"], `${at}.value.location`), offset: row1["offset"] === undefined ? undefined : parsePoint3(row1["offset"], `${at}.value.offset`), restraints: row1["restraints"] === undefined ? undefined : parseRestraints(row1["restraints"], `${at}.value.restraints`) }))(bimModelDiffGuardClosed(bimModelDiffGuardObject(row0["value"], `${at}.value`), `${at}.value`, ["name", "member", "location", "offset", "restraints"])) }; } })(bimModelDiffGuardObject(value, `${at}`));
}

export function parseStructuralSupportPatch(value: unknown, at = "$"): StructuralSupportPatch {
  const row = bimModelDiffGuardClosed(bimModelDiffGuardObject(value, at), at, ["name", "member", "location", "offset", "restraints"]);
  return {
    name: row["name"] === undefined ? undefined : bimModelDiffGuardString(row["name"], `${at}.name`),
    member: row["member"] === undefined ? undefined : bimModelDiffGuardString(row["member"], `${at}.member`),
    location: row["location"] === undefined ? undefined : parseStructuralLocation(row["location"], `${at}.location`),
    offset: row["offset"] === undefined ? undefined : parsePoint3(row["offset"], `${at}.offset`),
    restraints: row["restraints"] === undefined ? undefined : parseRestraints(row["restraints"], `${at}.restraints`),
  };
}

export function parseTagEntry(value: unknown, at = "$"): TagEntry {
  return ((row0) => { switch (bimModelDiffGuardMember(row0["entry"], `${at}.entry`, ["Created", "Deleted", "Replaced", "Patched"] as const)) { case "Created": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Created", value: parseTag(row0["value"], `${at}.value`) }; case "Deleted": bimModelDiffGuardClosed(row0, `${at}`, ["entry"]); return { entry: "Deleted" }; case "Replaced": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Replaced", value: parseTag(row0["value"], `${at}.value`) }; case "Patched": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Patched", value: parseTagPatch(row0["value"], `${at}.value`) }; } })(bimModelDiffGuardObject(value, `${at}`));
}

export function parseTagPatch(value: unknown, at = "$"): TagPatch {
  const row = bimModelDiffGuardClosed(bimModelDiffGuardObject(value, at), at, ["storey", "element", "category", "offset", "style"]);
  return {
    storey: row["storey"] === undefined ? undefined : bimModelDiffGuardString(row["storey"], `${at}.storey`),
    element: row["element"] === undefined ? undefined : bimModelDiffGuardString(row["element"], `${at}.element`),
    category: row["category"] === undefined ? undefined : parseTagCategory(row["category"], `${at}.category`),
    offset: row["offset"] === undefined ? undefined : parsePoint2(row["offset"], `${at}.offset`),
    style: row["style"] === undefined ? undefined : bimModelDiffGuardString(row["style"], `${at}.style`),
  };
}

export function parseTextNoteEntry(value: unknown, at = "$"): TextNoteEntry {
  return ((row0) => { switch (bimModelDiffGuardMember(row0["entry"], `${at}.entry`, ["Created", "Deleted", "Replaced", "Patched"] as const)) { case "Created": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Created", value: parseTextNote(row0["value"], `${at}.value`) }; case "Deleted": bimModelDiffGuardClosed(row0, `${at}`, ["entry"]); return { entry: "Deleted" }; case "Replaced": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Replaced", value: parseTextNote(row0["value"], `${at}.value`) }; case "Patched": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Patched", value: parseTextNotePatch(row0["value"], `${at}.value`) }; } })(bimModelDiffGuardObject(value, `${at}`));
}

export function parseTextNotePatch(value: unknown, at = "$"): TextNotePatch {
  const row = bimModelDiffGuardClosed(bimModelDiffGuardObject(value, at), at, ["storey", "position", "text", "rotation", "style"]);
  return {
    storey: row["storey"] === undefined ? undefined : bimModelDiffGuardString(row["storey"], `${at}.storey`),
    position: row["position"] === undefined ? undefined : parsePoint2(row["position"], `${at}.position`),
    text: row["text"] === undefined ? undefined : bimModelDiffGuardString(row["text"], `${at}.text`),
    rotation: row["rotation"] === undefined ? undefined : bimModelDiffGuardNumber(row["rotation"], `${at}.rotation`),
    style: row["style"] === undefined ? undefined : bimModelDiffGuardString(row["style"], `${at}.style`),
  };
}

export function parseViewEntry(value: unknown, at = "$"): ViewEntry {
  return ((row0) => { switch (bimModelDiffGuardMember(row0["entry"], `${at}.entry`, ["Created", "Deleted", "Replaced", "Patched"] as const)) { case "Created": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Created", value: parseView(row0["value"], `${at}.value`) }; case "Deleted": bimModelDiffGuardClosed(row0, `${at}`, ["entry"]); return { entry: "Deleted" }; case "Replaced": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Replaced", value: parseView(row0["value"], `${at}.value`) }; case "Patched": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Patched", value: parseViewPatch(row0["value"], `${at}.value`) }; } })(bimModelDiffGuardObject(value, `${at}`));
}

export function parseViewPatch(value: unknown, at = "$"): ViewPatch {
  const row = bimModelDiffGuardClosed(bimModelDiffGuardObject(value, at), at, ["building", "name", "kind", "storey", "plane", "camera", "cut_height", "depth", "crop", "hidden", "phase", "scale", "detail"]);
  return {
    building: row["building"] === undefined ? undefined : bimModelDiffGuardString(row["building"], `${at}.building`),
    name: row["name"] === undefined ? undefined : bimModelDiffGuardString(row["name"], `${at}.name`),
    kind: row["kind"] === undefined ? undefined : parseViewKind(row["kind"], `${at}.kind`),
    storey: row["storey"] === undefined ? undefined : ((row0) => ({ value: (row0["value"] === null ? null : bimModelDiffGuardString(row0["value"], `${at}.storey.value`)) }))(bimModelDiffGuardClosed(bimModelDiffGuardObject(row["storey"], `${at}.storey`), `${at}.storey`, ["value"])),
    plane: row["plane"] === undefined ? undefined : ((row0) => ({ value: (row0["value"] === null ? null : parseViewPlane(row0["value"], `${at}.plane.value`)) }))(bimModelDiffGuardClosed(bimModelDiffGuardObject(row["plane"], `${at}.plane`), `${at}.plane`, ["value"])),
    camera: row["camera"] === undefined ? undefined : ((row0) => ({ value: (row0["value"] === null ? null : parseViewCamera(row0["value"], `${at}.camera.value`)) }))(bimModelDiffGuardClosed(bimModelDiffGuardObject(row["camera"], `${at}.camera`), `${at}.camera`, ["value"])),
    cut_height: row["cut_height"] === undefined ? undefined : ((row0) => ({ value: (row0["value"] === null ? null : bimModelDiffGuardNumber(row0["value"], `${at}.cut_height.value`)) }))(bimModelDiffGuardClosed(bimModelDiffGuardObject(row["cut_height"], `${at}.cut_height`), `${at}.cut_height`, ["value"])),
    depth: row["depth"] === undefined ? undefined : bimModelDiffGuardNumber(row["depth"], `${at}.depth`),
    crop: row["crop"] === undefined ? undefined : ((row0) => ({ value: (row0["value"] === null ? null : parseViewCrop(row0["value"], `${at}.crop.value`)) }))(bimModelDiffGuardClosed(bimModelDiffGuardObject(row["crop"], `${at}.crop`), `${at}.crop`, ["value"])),
    hidden: row["hidden"] === undefined ? undefined : bimModelDiffGuardArray(row["hidden"], `${at}.hidden`).map((item, index) => parseViewCategory(item, `${at}.hidden[${index}]`)),
    phase: row["phase"] === undefined ? undefined : ((row0) => ({ value: (row0["value"] === null ? null : parsePhase(row0["value"], `${at}.phase.value`)) }))(bimModelDiffGuardClosed(bimModelDiffGuardObject(row["phase"], `${at}.phase`), `${at}.phase`, ["value"])),
    scale: row["scale"] === undefined ? undefined : bimModelDiffGuardInteger(row["scale"], `${at}.scale`),
    detail: row["detail"] === undefined ? undefined : parseDetailLevel(row["detail"], `${at}.detail`),
  };
}

export function parseViewportEntry(value: unknown, at = "$"): ViewportEntry {
  return ((row0) => { switch (bimModelDiffGuardMember(row0["entry"], `${at}.entry`, ["Created", "Deleted", "Replaced", "Patched"] as const)) { case "Created": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Created", value: parseViewport(row0["value"], `${at}.value`) }; case "Deleted": bimModelDiffGuardClosed(row0, `${at}`, ["entry"]); return { entry: "Deleted" }; case "Replaced": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Replaced", value: parseViewport(row0["value"], `${at}.value`) }; case "Patched": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Patched", value: parseViewportPatch(row0["value"], `${at}.value`) }; } })(bimModelDiffGuardObject(value, `${at}`));
}

export function parseViewportPatch(value: unknown, at = "$"): ViewportPatch {
  const row = bimModelDiffGuardClosed(bimModelDiffGuardObject(value, at), at, ["sheet", "view", "position", "scale", "crop", "label"]);
  return {
    sheet: row["sheet"] === undefined ? undefined : bimModelDiffGuardString(row["sheet"], `${at}.sheet`),
    view: row["view"] === undefined ? undefined : bimModelDiffGuardString(row["view"], `${at}.view`),
    position: row["position"] === undefined ? undefined : parsePoint2(row["position"], `${at}.position`),
    scale: row["scale"] === undefined ? undefined : bimModelDiffGuardInteger(row["scale"], `${at}.scale`),
    crop: row["crop"] === undefined ? undefined : ((row0) => ({ value: (row0["value"] === null ? null : parseViewCrop(row0["value"], `${at}.crop.value`)) }))(bimModelDiffGuardClosed(bimModelDiffGuardObject(row["crop"], `${at}.crop`), `${at}.crop`, ["value"])),
    label: row["label"] === undefined ? undefined : ((row0) => ({ value: (row0["value"] === null ? null : bimModelDiffGuardString(row0["value"], `${at}.label.value`)) }))(bimModelDiffGuardClosed(bimModelDiffGuardObject(row["label"], `${at}.label`), `${at}.label`, ["value"])),
  };
}

export function parseWallEntry(value: unknown, at = "$"): WallEntry {
  return ((row0) => { switch (bimModelDiffGuardMember(row0["entry"], `${at}.entry`, ["Created", "Deleted", "Replaced", "Patched"] as const)) { case "Created": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Created", value: parseWall(row0["value"], `${at}.value`) }; case "Deleted": bimModelDiffGuardClosed(row0, `${at}`, ["entry"]); return { entry: "Deleted" }; case "Replaced": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Replaced", value: parseWall(row0["value"], `${at}.value`) }; case "Patched": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Patched", value: parseWallPatch(row0["value"], `${at}.value`) }; } })(bimModelDiffGuardObject(value, `${at}`));
}

export function parseWallPatch(value: unknown, at = "$"): WallPatch {
  const row = bimModelDiffGuardClosed(bimModelDiffGuardObject(value, at), at, ["storey", "wall_type", "axis", "location", "base_offset", "top", "phase", "start_join", "end_join", "name", "base_slab"]);
  return {
    storey: row["storey"] === undefined ? undefined : bimModelDiffGuardString(row["storey"], `${at}.storey`),
    wall_type: row["wall_type"] === undefined ? undefined : bimModelDiffGuardString(row["wall_type"], `${at}.wall_type`),
    axis: row["axis"] === undefined ? undefined : parseAxis(row["axis"], `${at}.axis`),
    location: row["location"] === undefined ? undefined : parseLocationLine(row["location"], `${at}.location`),
    base_offset: row["base_offset"] === undefined ? undefined : bimModelDiffGuardNumber(row["base_offset"], `${at}.base_offset`),
    top: row["top"] === undefined ? undefined : parseTopConstraint(row["top"], `${at}.top`),
    phase: row["phase"] === undefined ? undefined : parsePhase(row["phase"], `${at}.phase`),
    start_join: row["start_join"] === undefined ? undefined : ((row0) => ({ value: (row0["value"] === null ? null : parseEndJoin(row0["value"], `${at}.start_join.value`)) }))(bimModelDiffGuardClosed(bimModelDiffGuardObject(row["start_join"], `${at}.start_join`), `${at}.start_join`, ["value"])),
    end_join: row["end_join"] === undefined ? undefined : ((row0) => ({ value: (row0["value"] === null ? null : parseEndJoin(row0["value"], `${at}.end_join.value`)) }))(bimModelDiffGuardClosed(bimModelDiffGuardObject(row["end_join"], `${at}.end_join`), `${at}.end_join`, ["value"])),
    name: row["name"] === undefined ? undefined : bimModelDiffGuardString(row["name"], `${at}.name`),
    base_slab: row["base_slab"] === undefined ? undefined : ((row0) => ({ value: (row0["value"] === null ? null : bimModelDiffGuardString(row0["value"], `${at}.base_slab.value`)) }))(bimModelDiffGuardClosed(bimModelDiffGuardObject(row["base_slab"], `${at}.base_slab`), `${at}.base_slab`, ["value"])),
  };
}

export function parseWallSweepEntry(value: unknown, at = "$"): WallSweepEntry {
  return ((row0) => { switch (bimModelDiffGuardMember(row0["entry"], `${at}.entry`, ["Created", "Deleted", "Replaced", "Patched"] as const)) { case "Created": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Created", value: parseWallSweep(row0["value"], `${at}.value`) }; case "Deleted": bimModelDiffGuardClosed(row0, `${at}`, ["entry"]); return { entry: "Deleted" }; case "Replaced": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Replaced", value: parseWallSweep(row0["value"], `${at}.value`) }; case "Patched": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Patched", value: parseWallSweepPatch(row0["value"], `${at}.value`) }; } })(bimModelDiffGuardObject(value, `${at}`));
}

export function parseWallSweepPatch(value: unknown, at = "$"): WallSweepPatch {
  const row = bimModelDiffGuardClosed(bimModelDiffGuardObject(value, at), at, ["host", "side", "profile", "height", "inset", "material", "name"]);
  return {
    host: row["host"] === undefined ? undefined : bimModelDiffGuardString(row["host"], `${at}.host`),
    side: row["side"] === undefined ? undefined : parseWallSide(row["side"], `${at}.side`),
    profile: row["profile"] === undefined ? undefined : parseProfile(row["profile"], `${at}.profile`),
    height: row["height"] === undefined ? undefined : bimModelDiffGuardNumber(row["height"], `${at}.height`),
    inset: row["inset"] === undefined ? undefined : bimModelDiffGuardNumber(row["inset"], `${at}.inset`),
    material: row["material"] === undefined ? undefined : bimModelDiffGuardString(row["material"], `${at}.material`),
    name: row["name"] === undefined ? undefined : bimModelDiffGuardString(row["name"], `${at}.name`),
  };
}

export function parseWallTypeEntry(value: unknown, at = "$"): WallTypeEntry {
  return ((row0) => { switch (bimModelDiffGuardMember(row0["entry"], `${at}.entry`, ["Created", "Deleted", "Replaced", "Patched"] as const)) { case "Created": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Created", value: parseWallType(row0["value"], `${at}.value`) }; case "Deleted": bimModelDiffGuardClosed(row0, `${at}`, ["entry"]); return { entry: "Deleted" }; case "Replaced": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Replaced", value: parseWallType(row0["value"], `${at}.value`) }; case "Patched": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Patched", value: parseWallTypePatch(row0["value"], `${at}.value`) }; } })(bimModelDiffGuardObject(value, `${at}`));
}

export function parseWallTypePatch(value: unknown, at = "$"): WallTypePatch {
  const row = bimModelDiffGuardClosed(bimModelDiffGuardObject(value, at), at, ["name", "layers"]);
  return {
    name: row["name"] === undefined ? undefined : bimModelDiffGuardString(row["name"], `${at}.name`),
    layers: row["layers"] === undefined ? undefined : bimModelDiffGuardArray(row["layers"], `${at}.layers`).map((item, index) => parseLayer(item, `${at}.layers[${index}]`)),
  };
}

export function parseWindowTypeEntry(value: unknown, at = "$"): WindowTypeEntry {
  return ((row0) => { switch (bimModelDiffGuardMember(row0["entry"], `${at}.entry`, ["Created", "Deleted", "Replaced", "Patched"] as const)) { case "Created": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Created", value: parseWindowType(row0["value"], `${at}.value`) }; case "Deleted": bimModelDiffGuardClosed(row0, `${at}`, ["entry"]); return { entry: "Deleted" }; case "Replaced": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Replaced", value: parseWindowType(row0["value"], `${at}.value`) }; case "Patched": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Patched", value: parseWindowTypePatch(row0["value"], `${at}.value`) }; } })(bimModelDiffGuardObject(value, `${at}`));
}

export function parseWindowTypePatch(value: unknown, at = "$"): WindowTypePatch {
  const row = bimModelDiffGuardClosed(bimModelDiffGuardObject(value, at), at, ["name", "width", "height", "sill", "frame_width", "frame_depth", "panes", "material", "u_value", "g_value", "frame_fraction"]);
  return {
    name: row["name"] === undefined ? undefined : bimModelDiffGuardString(row["name"], `${at}.name`),
    width: row["width"] === undefined ? undefined : bimModelDiffGuardNumber(row["width"], `${at}.width`),
    height: row["height"] === undefined ? undefined : bimModelDiffGuardNumber(row["height"], `${at}.height`),
    sill: row["sill"] === undefined ? undefined : bimModelDiffGuardNumber(row["sill"], `${at}.sill`),
    frame_width: row["frame_width"] === undefined ? undefined : bimModelDiffGuardNumber(row["frame_width"], `${at}.frame_width`),
    frame_depth: row["frame_depth"] === undefined ? undefined : bimModelDiffGuardNumber(row["frame_depth"], `${at}.frame_depth`),
    panes: row["panes"] === undefined ? undefined : bimModelDiffGuardInteger(row["panes"], `${at}.panes`),
    material: row["material"] === undefined ? undefined : bimModelDiffGuardString(row["material"], `${at}.material`),
    u_value: row["u_value"] === undefined ? undefined : ((row0) => ({ value: (row0["value"] === null ? null : bimModelDiffGuardNumber(row0["value"], `${at}.u_value.value`)) }))(bimModelDiffGuardClosed(bimModelDiffGuardObject(row["u_value"], `${at}.u_value`), `${at}.u_value`, ["value"])),
    g_value: row["g_value"] === undefined ? undefined : ((row0) => ({ value: (row0["value"] === null ? null : bimModelDiffGuardNumber(row0["value"], `${at}.g_value.value`)) }))(bimModelDiffGuardClosed(bimModelDiffGuardObject(row["g_value"], `${at}.g_value`), `${at}.g_value`, ["value"])),
    frame_fraction: row["frame_fraction"] === undefined ? undefined : ((row0) => ({ value: (row0["value"] === null ? null : bimModelDiffGuardNumber(row0["value"], `${at}.frame_fraction.value`)) }))(bimModelDiffGuardClosed(bimModelDiffGuardObject(row["frame_fraction"], `${at}.frame_fraction`), `${at}.frame_fraction`, ["value"])),
  };
}

export function parseZoneEntry(value: unknown, at = "$"): ZoneEntry {
  return ((row0) => { switch (bimModelDiffGuardMember(row0["entry"], `${at}.entry`, ["Created", "Deleted", "Replaced", "Patched"] as const)) { case "Created": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Created", value: parseZone(row0["value"], `${at}.value`) }; case "Deleted": bimModelDiffGuardClosed(row0, `${at}`, ["entry"]); return { entry: "Deleted" }; case "Replaced": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Replaced", value: parseZone(row0["value"], `${at}.value`) }; case "Patched": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Patched", value: parseZonePatch(row0["value"], `${at}.value`) }; } })(bimModelDiffGuardObject(value, `${at}`));
}

export function parseZonePatch(value: unknown, at = "$"): ZonePatch {
  const row = bimModelDiffGuardClosed(bimModelDiffGuardObject(value, at), at, ["name", "category", "occupancy_density"]);
  return {
    name: row["name"] === undefined ? undefined : bimModelDiffGuardString(row["name"], `${at}.name`),
    category: row["category"] === undefined ? undefined : bimModelDiffGuardString(row["category"], `${at}.category`),
    occupancy_density: row["occupancy_density"] === undefined ? undefined : bimModelDiffGuardNumber(row["occupancy_density"], `${at}.occupancy_density`),
  };
}

export function parseDesignOptionEntry(value: unknown, at = "$"): DesignOptionEntry {
  return ((row0) => { switch (bimModelDiffGuardMember(row0["entry"], `${at}.entry`, ["Created", "Deleted", "Replaced", "Patched"] as const)) { case "Created": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Created", value: parseDesignOption(row0["value"], `${at}.value`) }; case "Deleted": bimModelDiffGuardClosed(row0, `${at}`, ["entry"]); return { entry: "Deleted" }; case "Replaced": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Replaced", value: parseDesignOption(row0["value"], `${at}.value`) }; case "Patched": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Patched", value: parseDesignOptionPatch(row0["value"], `${at}.value`) }; } })(bimModelDiffGuardObject(value, `${at}`));
}

export function parseDesignOptionPatch(value: unknown, at = "$"): DesignOptionPatch {
  const row = bimModelDiffGuardClosed(bimModelDiffGuardObject(value, at), at, ["group", "name", "primary"]);
  return {
    group: row["group"] === undefined ? undefined : bimModelDiffGuardString(row["group"], `${at}.group`),
    name: row["name"] === undefined ? undefined : bimModelDiffGuardString(row["name"], `${at}.name`),
    primary: row["primary"] === undefined ? undefined : bimModelDiffGuardBoolean(row["primary"], `${at}.primary`),
  };
}

export function parseElementMembershipEntry(value: unknown, at = "$"): ElementMembershipEntry {
  return ((row0) => { switch (bimModelDiffGuardMember(row0["entry"], `${at}.entry`, ["Created", "Deleted", "Replaced", "Patched"] as const)) { case "Created": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Created", value: parseElementMembership(row0["value"], `${at}.value`) }; case "Deleted": bimModelDiffGuardClosed(row0, `${at}`, ["entry"]); return { entry: "Deleted" }; case "Replaced": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Replaced", value: parseElementMembership(row0["value"], `${at}.value`) }; case "Patched": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Patched", value: parseElementMembershipPatch(row0["value"], `${at}.value`) }; } })(bimModelDiffGuardObject(value, `${at}`));
}

export function parseElementMembershipPatch(value: unknown, at = "$"): ElementMembershipPatch {
  const row = bimModelDiffGuardClosed(bimModelDiffGuardObject(value, at), at, ["target"]);
  return {
    target: row["target"] === undefined ? undefined : bimModelDiffGuardString(row["target"], `${at}.target`),
  };
}

export function parseOptionGroupEntry(value: unknown, at = "$"): OptionGroupEntry {
  return ((row0) => { switch (bimModelDiffGuardMember(row0["entry"], `${at}.entry`, ["Created", "Deleted", "Replaced", "Patched"] as const)) { case "Created": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Created", value: parseOptionGroup(row0["value"], `${at}.value`) }; case "Deleted": bimModelDiffGuardClosed(row0, `${at}`, ["entry"]); return { entry: "Deleted" }; case "Replaced": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Replaced", value: parseOptionGroup(row0["value"], `${at}.value`) }; case "Patched": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Patched", value: parseOptionGroupPatch(row0["value"], `${at}.value`) }; } })(bimModelDiffGuardObject(value, `${at}`));
}

export function parseOptionGroupPatch(value: unknown, at = "$"): OptionGroupPatch {
  const row = bimModelDiffGuardClosed(bimModelDiffGuardObject(value, at), at, ["name"]);
  return {
    name: row["name"] === undefined ? undefined : bimModelDiffGuardString(row["name"], `${at}.name`),
  };
}

export function parseWorksetEntry(value: unknown, at = "$"): WorksetEntry {
  return ((row0) => { switch (bimModelDiffGuardMember(row0["entry"], `${at}.entry`, ["Created", "Deleted", "Replaced", "Patched"] as const)) { case "Created": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Created", value: parseWorkset(row0["value"], `${at}.value`) }; case "Deleted": bimModelDiffGuardClosed(row0, `${at}`, ["entry"]); return { entry: "Deleted" }; case "Replaced": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Replaced", value: parseWorkset(row0["value"], `${at}.value`) }; case "Patched": bimModelDiffGuardClosed(row0, `${at}`, ["entry", "value"]); return { entry: "Patched", value: parseWorksetPatch(row0["value"], `${at}.value`) }; } })(bimModelDiffGuardObject(value, `${at}`));
}

export function parseWorksetPatch(value: unknown, at = "$"): WorksetPatch {
  const row = bimModelDiffGuardClosed(bimModelDiffGuardObject(value, at), at, ["name", "default_visible"]);
  return {
    name: row["name"] === undefined ? undefined : bimModelDiffGuardString(row["name"], `${at}.name`),
    default_visible: row["default_visible"] === undefined ? undefined : bimModelDiffGuardBoolean(row["default_visible"], `${at}.default_visible`),
  };
}
