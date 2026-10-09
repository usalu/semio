/** 🏙️ BIM model direct-mutation discriminated union. */
import type { AnnotationAnchor, AnnotationStyle, AreaMeasure, AreaScheme, Axis, Baluster, Beam, BeamType, Building, Ceiling, CeilingType, ClassificationItem, ClassificationSystem, Column, ColumnType, CurtainGrid, CurtainPanel, CurtainPanelOverride, CurtainWall, CurtainWallType, DetailLevel, Dimension, DimensionUnit, DoorLeaves, DoorType, EndJoin, ExprPoint3, Family, FamilyCategory, FamilySolid, GridLine, Infill, Layer, Leader, LocationLine, Material, MaterialCategory, Opening, OpeningKind, Orientation, Paper, ParameterKind, Phase, Point2, Profile, PropertyDef, PropertyTemplate, PropertyValue, Railing, RailingHost, Ramp, Rgb, RiserKind, Roof, RoofShape, RoofType, Schedule, ScheduleCategory, ScheduleColumn, ScheduleFilter, ScheduleGroup, ScheduleSort, Sheet, SheetRevision, Site, Slab, SlabType, Slope, SolidShape, Space, SpaceBoundary, Stair, StairFlight, StairStringer, Storey, Swing, Tag, TagCategory, TemplateTarget, Terminator, TextNote, TopConstraint, Vertex, View, ViewCamera, ViewCategory, ViewCrop, ViewPlane, Viewport, Wall, WallSide, WallSweep, WallType, WindowType, Zone } from "../🟦️.ts";

export interface CreateSite {
  mutation: "createSite";
  id: string;
  site: Site;
}

export interface DeleteSite {
  mutation: "deleteSite";
  id: string;
}

export interface CreateBuilding {
  mutation: "createBuilding";
  id: string;
  building: Building;
}

export interface DeleteBuilding {
  mutation: "deleteBuilding";
  id: string;
}

export interface CreateStorey {
  mutation: "createStorey";
  id: string;
  storey: Storey;
}

export interface RenameStorey {
  mutation: "renameStorey";
  id: string;
  name: string;
}

export interface SetStoreyHeight {
  mutation: "setStoreyHeight";
  id: string;
  height: number;
}

export interface SetStoreyLevel {
  mutation: "setStoreyLevel";
  id: string;
  level: number;
}

export interface DeleteStorey {
  mutation: "deleteStorey";
  id: string;
}

export interface CreateWall {
  mutation: "createWall";
  id: string;
  wall: Wall;
}

export interface DeleteWall {
  mutation: "deleteWall";
  id: string;
}

export interface SetWallTop {
  mutation: "setWallTop";
  id: string;
  top: TopConstraint;
}

export interface SetStoreyCutHeight {
  mutation: "setStoreyCutHeight";
  id: string;
  cut_height: Record<string, unknown>;
}

export interface CreateMaterial {
  mutation: "createMaterial";
  id: string;
  material: Material;
}

export interface DeleteMaterial {
  mutation: "deleteMaterial";
  id: string;
}

export interface SetMaterial {
  mutation: "setMaterial";
  id: string;
  name?: string;
  category?: MaterialCategory;
  color?: Rgb;
  density?: number;
  conductivity?: number;
  specific_heat?: number;
}

export interface CreateWallType {
  mutation: "createWallType";
  id: string;
  wall_type: WallType;
}

export interface DeleteWallType {
  mutation: "deleteWallType";
  id: string;
}

export interface SetWallType {
  mutation: "setWallType";
  id: string;
  name?: string;
  layers?: Layer[];
}

export interface CreateSlabType {
  mutation: "createSlabType";
  id: string;
  slab_type: SlabType;
}

export interface DeleteSlabType {
  mutation: "deleteSlabType";
  id: string;
}

export interface SetSlabType {
  mutation: "setSlabType";
  id: string;
  name?: string;
  layers?: Layer[];
}

export interface CreateRoofType {
  mutation: "createRoofType";
  id: string;
  roof_type: RoofType;
}

export interface DeleteRoofType {
  mutation: "deleteRoofType";
  id: string;
}

export interface SetRoofType {
  mutation: "setRoofType";
  id: string;
  name?: string;
  layers?: Layer[];
}

export interface CreateColumnType {
  mutation: "createColumnType";
  id: string;
  column_type: ColumnType;
}

export interface DeleteColumnType {
  mutation: "deleteColumnType";
  id: string;
}

export interface SetColumnType {
  mutation: "setColumnType";
  id: string;
  name?: string;
  profile?: Profile;
  material?: string;
}

export interface CreateBeamType {
  mutation: "createBeamType";
  id: string;
  beam_type: BeamType;
}

export interface DeleteBeamType {
  mutation: "deleteBeamType";
  id: string;
}

export interface SetBeamType {
  mutation: "setBeamType";
  id: string;
  name?: string;
  profile?: Profile;
  material?: string;
}

export interface CreateWindowType {
  mutation: "createWindowType";
  id: string;
  window_type: WindowType;
}

export interface DeleteWindowType {
  mutation: "deleteWindowType";
  id: string;
}

export interface SetWindowType {
  mutation: "setWindowType";
  id: string;
  name?: string;
  width?: number;
  height?: number;
  sill?: number;
  frame_width?: number;
  frame_depth?: number;
  panes?: number;
  material?: string;
}

export interface CreateDoorType {
  mutation: "createDoorType";
  id: string;
  door_type: DoorType;
}

export interface DeleteDoorType {
  mutation: "deleteDoorType";
  id: string;
}

export interface SetDoorType {
  mutation: "setDoorType";
  id: string;
  name?: string;
  width?: number;
  height?: number;
  frame_width?: number;
  frame_depth?: number;
  leaves?: DoorLeaves;
  swing?: Swing;
  material?: string;
}

export interface SetProjectInfo {
  mutation: "setProjectInfo";
  name?: string;
  description?: string;
  author?: string;
  organization?: string;
  phase_names?: string[];
}

export interface SetSite {
  mutation: "setSite";
  id: string;
  name?: string;
  latitude?: number;
  longitude?: number;
  elevation?: number;
  true_north?: number;
  boundary?: Point2[];
}

export interface SetBuilding {
  mutation: "setBuilding";
  id: string;
  name?: string;
  origin?: Point2;
  rotation?: number;
  elevation?: number;
}

export interface CreateGridLine {
  mutation: "createGridLine";
  id: string;
  grid_line: GridLine;
}

export interface DeleteGridLine {
  mutation: "deleteGridLine";
  id: string;
}

export interface SetGridLine {
  mutation: "setGridLine";
  id: string;
  label?: string;
  start?: Point2;
  end?: Point2;
}

export interface SetWallAxis {
  mutation: "setWallAxis";
  id: string;
  axis: Axis;
}

export interface SetWallBaseOffset {
  mutation: "setWallBaseOffset";
  id: string;
  base_offset: number;
}

export interface SetWallTypeOf {
  mutation: "setWallTypeOf";
  id: string;
  wall_type: string;
}

export interface SetWallLocation {
  mutation: "setWallLocation";
  id: string;
  location: LocationLine;
}

export interface FlipWall {
  mutation: "flipWall";
  id: string;
}

export interface SplitWall {
  mutation: "splitWall";
  id: string;
  t: number;
  new_id: string;
}

export interface CreateCurtainWall {
  mutation: "createCurtainWall";
  id: string;
  curtain_wall: CurtainWall;
}

export interface DeleteCurtainWall {
  mutation: "deleteCurtainWall";
  id: string;
}

export interface SetCurtainWall {
  mutation: "setCurtainWall";
  id: string;
  axis?: Axis;
  base_offset?: number;
  top?: TopConstraint;
  name?: string;
}

export interface CreateColumn {
  mutation: "createColumn";
  id: string;
  column: Column;
}

export interface DeleteColumn {
  mutation: "deleteColumn";
  id: string;
}

export interface SetColumn {
  mutation: "setColumn";
  id: string;
  column_type?: string;
  position?: Point2;
  rotation?: number;
  base_offset?: number;
  top?: TopConstraint;
  name?: string;
}

export interface CreateBeam {
  mutation: "createBeam";
  id: string;
  beam: Beam;
}

export interface DeleteBeam {
  mutation: "deleteBeam";
  id: string;
}

export interface SetBeam {
  mutation: "setBeam";
  id: string;
  beam_type?: string;
  top_offset?: number;
  end_top_offset?: Record<string, unknown>;
  name?: string;
}

export interface CreateSlab {
  mutation: "createSlab";
  id: string;
  slab: Slab;
}

export interface DeleteSlab {
  mutation: "deleteSlab";
  id: string;
}

export interface SetSlabBoundary {
  mutation: "setSlabBoundary";
  id: string;
  boundary: Vertex[];
  holes: Vertex[][];
}

export interface SetSlab {
  mutation: "setSlab";
  id: string;
  slab_type?: string;
  offset?: number;
  slope?: Slope;
  name?: string;
}

export interface CreateRoof {
  mutation: "createRoof";
  id: string;
  roof: Roof;
}

export interface DeleteRoof {
  mutation: "deleteRoof";
  id: string;
}

export interface SetRoofFootprint {
  mutation: "setRoofFootprint";
  id: string;
  footprint: Vertex[];
}

export interface SetRoofShape {
  mutation: "setRoofShape";
  id: string;
  shape?: RoofShape;
  overhang?: number;
  base_offset?: number;
}

export interface CreateOpening {
  mutation: "createOpening";
  id: string;
  opening: Opening;
}

export interface DeleteOpening {
  mutation: "deleteOpening";
  id: string;
}

export interface MoveOpening {
  mutation: "moveOpening";
  id: string;
  offset: number;
  host?: string;
}

export interface SetOpening {
  mutation: "setOpening";
  id: string;
  kind?: OpeningKind;
  sill_override?: Record<string, unknown>;
  width?: Record<string, unknown>;
  height?: Record<string, unknown>;
  flip_hand?: boolean;
  flip_facing?: boolean;
  name?: string;
  reveal_depth?: Record<string, unknown>;
  reveal_material?: Record<string, unknown>;
}

export interface CreateStair {
  mutation: "createStair";
  id: string;
  stair: Stair;
}

export interface DeleteStair {
  mutation: "deleteStair";
  id: string;
}

export interface SetStair {
  mutation: "setStair";
  id: string;
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

export interface CreateRailing {
  mutation: "createRailing";
  id: string;
  railing: Railing;
}

export interface DeleteRailing {
  mutation: "deleteRailing";
  id: string;
}

export interface SetRailing {
  mutation: "setRailing";
  id: string;
  path?: Point2[];
  height?: number;
  post_spacing?: number;
  profile?: Profile;
  post_profile?: Profile;
  baluster?: Baluster;
  infill?: Infill;
  host?: RailingHost;
  material?: string;
  base_offset?: number;
  name?: string;
}

export interface CreateSpace {
  mutation: "createSpace";
  id: string;
  space: Space;
}

export interface DeleteSpace {
  mutation: "deleteSpace";
  id: string;
}

export interface SetSpace {
  mutation: "setSpace";
  id: string;
  number?: string;
  name?: string;
  boundary?: SpaceBoundary;
  usage?: string;
  zone?: Record<string, unknown>;
  floor_finish?: Record<string, unknown>;
  wall_finish?: Record<string, unknown>;
  ceiling_finish?: Record<string, unknown>;
}

export interface MoveElements {
  mutation: "moveElements";
  ids: string[];
  vector: Point2;
}

export interface RotateElements {
  mutation: "rotateElements";
  ids: string[];
  pivot: Point2;
  angle: number;
}

export interface DeleteElements {
  mutation: "deleteElements";
  ids: string[];
}

export interface RenameElement {
  mutation: "renameElement";
  id: string;
  name: string;
}

export interface SetElementProperty {
  mutation: "setElementProperty";
  id: string;
  pset: string;
  property: string;
  value: PropertyValue;
}

export interface RemoveElementProperty {
  mutation: "removeElementProperty";
  id: string;
  pset: string;
  property: string;
}

export interface SetElementClassification {
  mutation: "setElementClassification";
  id: string;
  system: string;
  code: string;
}

export interface RemoveElementClassification {
  mutation: "removeElementClassification";
  id: string;
  system: string;
}

export interface PlaceElements {
  mutation: "placeElements";
  placements: Record<string, unknown>;
}

export interface SetElementStorey {
  mutation: "setElementStorey";
  id: string;
  storey: string;
}

export interface SetElementPhase {
  mutation: "setElementPhase";
  id: string;
  phase: Phase;
}

export interface SetWallEndJoin {
  mutation: "setWallEndJoin";
  id: string;
  end: string;
  join?: EndJoin;
}

export interface CopyElements {
  mutation: "copyElements";
  ids: string[];
  vector: Point2;
  prefix: string;
}

export interface MirrorElements {
  mutation: "mirrorElements";
  ids: string[];
  line_start: Point2;
  line_end: Point2;
  prefix?: string;
}

export interface ArrayElements {
  mutation: "arrayElements";
  ids: string[];
  prefix: string;
  pattern: Record<string, unknown>;
}

export interface AlignElements {
  mutation: "alignElements";
  ids: string[];
  axis: string;
  edge: string;
  target: number;
}

export interface OffsetWall {
  mutation: "offsetWall";
  id: string;
  new_id: string;
  distance: number;
}

export interface TrimExtendWall {
  mutation: "trimExtendWall";
  id: string;
  end: string;
  target: string;
}

export interface SplitSlab {
  mutation: "splitSlab";
  id: string;
  new_id: string;
  line_start: Point2;
  line_end: Point2;
}

export interface SplitBeam {
  mutation: "splitBeam";
  id: string;
  t: number;
  new_id: string;
}

export interface CreateCeilingType {
  mutation: "createCeilingType";
  id: string;
  ceiling_type: CeilingType;
}

export interface DeleteCeilingType {
  mutation: "deleteCeilingType";
  id: string;
}

export interface SetCeilingType {
  mutation: "setCeilingType";
  id: string;
  name?: string;
  layers?: Layer[];
}

export interface CreateCeiling {
  mutation: "createCeiling";
  id: string;
  ceiling: Ceiling;
}

export interface DeleteCeiling {
  mutation: "deleteCeiling";
  id: string;
}

export interface SetCeilingBoundary {
  mutation: "setCeilingBoundary";
  id: string;
  boundary: Vertex[];
  holes: Vertex[][];
}

export interface SetCeiling {
  mutation: "setCeiling";
  id: string;
  ceiling_type?: string;
  offset?: number;
  slope?: Slope;
  name?: string;
}

export interface CreateZone {
  mutation: "createZone";
  id: string;
  zone: Zone;
}

export interface SetZone {
  mutation: "setZone";
  id: string;
  name: string;
  category: string;
  occupancy_density: number;
}

export interface DeleteZone {
  mutation: "deleteZone";
  id: string;
}

export interface CreateAreaScheme {
  mutation: "createAreaScheme";
  id: string;
  area_scheme: AreaScheme;
}

export interface SetAreaScheme {
  mutation: "setAreaScheme";
  id: string;
  name: string;
  measure: AreaMeasure;
  usages: string[];
  zones: string[];
}

export interface DeleteAreaScheme {
  mutation: "deleteAreaScheme";
  id: string;
}

export interface CreateWallSweep {
  mutation: "createWallSweep";
  id: string;
  wall_sweep: WallSweep;
}

export interface SetWallSweep {
  mutation: "setWallSweep";
  id: string;
  host?: string;
  side?: WallSide;
  profile?: Profile;
  height?: number;
  inset?: number;
  material?: string;
  name?: string;
}

export interface DeleteWallSweep {
  mutation: "deleteWallSweep";
  id: string;
}

export interface SetWallBaseSlab {
  mutation: "setWallBaseSlab";
  id: string;
  slab?: string;
}

export interface CreateRamp {
  mutation: "createRamp";
  id: string;
  ramp: Ramp;
}

export interface SetRamp {
  mutation: "setRamp";
  id: string;
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

export interface DeleteRamp {
  mutation: "deleteRamp";
  id: string;
}

export interface CreateDimension {
  mutation: "createDimension";
  id: string;
  dimension: Dimension;
}

export interface DeleteDimension {
  mutation: "deleteDimension";
  id: string;
}

export interface SetDimension {
  mutation: "setDimension";
  id: string;
  anchors?: AnnotationAnchor[];
  angle?: number;
  offset?: number;
  style?: string;
  lock?: Record<string, unknown>;
  name?: string;
}

export interface CreateTag {
  mutation: "createTag";
  id: string;
  tag: Tag;
}

export interface DeleteTag {
  mutation: "deleteTag";
  id: string;
}

export interface SetTag {
  mutation: "setTag";
  id: string;
  element?: string;
  category?: TagCategory;
  offset?: Point2;
  style?: string;
}

export interface CreateTextNote {
  mutation: "createTextNote";
  id: string;
  text_note: TextNote;
}

export interface DeleteTextNote {
  mutation: "deleteTextNote";
  id: string;
}

export interface SetTextNote {
  mutation: "setTextNote";
  id: string;
  position?: Point2;
  text?: string;
  rotation?: number;
  style?: string;
}

export interface CreateLeader {
  mutation: "createLeader";
  id: string;
  leader: Leader;
}

export interface DeleteLeader {
  mutation: "deleteLeader";
  id: string;
}

export interface SetLeader {
  mutation: "setLeader";
  id: string;
  anchor?: AnnotationAnchor;
  offset?: Point2;
  text?: string;
  style?: string;
}

export interface CreateAnnotationStyle {
  mutation: "createAnnotationStyle";
  id: string;
  annotation_style: AnnotationStyle;
}

export interface DeleteAnnotationStyle {
  mutation: "deleteAnnotationStyle";
  id: string;
}

export interface SetAnnotationStyle {
  mutation: "setAnnotationStyle";
  id: string;
  name?: string;
  text_height?: number;
  terminator?: Terminator;
  unit?: DimensionUnit;
  precision?: number;
  mark_size?: number;
  gap?: number;
  overshoot?: number;
}

export interface CreateView {
  mutation: "createView";
  id: string;
  view: View;
}

export interface SetView {
  mutation: "setView";
  id: string;
  name?: string;
  storey?: string;
  plane?: ViewPlane;
  camera?: ViewCamera;
  cut_height?: Record<string, unknown>;
  depth?: number;
  crop?: ViewCrop;
  hidden?: ViewCategory[];
  phase?: Phase;
  scale?: number;
  detail?: DetailLevel;
}

export interface DeleteView {
  mutation: "deleteView";
  id: string;
}

export interface CreateSchedule {
  mutation: "createSchedule";
  id: string;
  schedule: Schedule;
}

export interface SetSchedule {
  mutation: "setSchedule";
  id: string;
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

export interface DeleteSchedule {
  mutation: "deleteSchedule";
  id: string;
}

export interface CreateSheet {
  mutation: "createSheet";
  id: string;
  sheet: Sheet;
}

export interface SetSheet {
  mutation: "setSheet";
  id: string;
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

export interface DeleteSheet {
  mutation: "deleteSheet";
  id: string;
}

export interface CreateViewport {
  mutation: "createViewport";
  id: string;
  viewport: Viewport;
}

export interface SetViewport {
  mutation: "setViewport";
  id: string;
  sheet?: string;
  view?: string;
  position?: Point2;
  scale?: number;
  crop?: ViewCrop;
  label?: Record<string, unknown>;
}

export interface DeleteViewport {
  mutation: "deleteViewport";
  id: string;
}

export interface CreateSheetRevision {
  mutation: "createSheetRevision";
  id: string;
  sheet_revision: SheetRevision;
}

export interface SetSheetRevision {
  mutation: "setSheetRevision";
  id: string;
  number?: string;
  date?: string;
  description?: string;
  author?: string;
}

export interface DeleteSheetRevision {
  mutation: "deleteSheetRevision";
  id: string;
}

export interface CreatePropertyTemplate {
  mutation: "createPropertyTemplate";
  id: string;
  template: PropertyTemplate;
}

export interface SetPropertyTemplate {
  mutation: "setPropertyTemplate";
  id: string;
  name?: string;
  applies_to?: TemplateTarget[];
  properties?: PropertyDef[];
}

export interface DeletePropertyTemplate {
  mutation: "deletePropertyTemplate";
  id: string;
}

export interface CreateClassificationSystem {
  mutation: "createClassificationSystem";
  id: string;
  system: ClassificationSystem;
}

export interface SetClassificationSystem {
  mutation: "setClassificationSystem";
  id: string;
  name?: string;
  edition?: string;
  source?: Record<string, unknown>;
  entries?: ClassificationItem[];
}

export interface DeleteClassificationSystem {
  mutation: "deleteClassificationSystem";
  id: string;
}

export interface SetBeamAxis {
  mutation: "setBeamAxis";
  id: string;
  axis: Axis;
}

export interface SetColumnTilt {
  mutation: "setColumnTilt";
  id: string;
  tilt?: Slope;
}

export interface CreateCurtainWallType {
  mutation: "createCurtainWallType";
  id: string;
  curtain_wall_type: CurtainWallType;
}

export interface SetCurtainWallType {
  mutation: "setCurtainWallType";
  id: string;
  name?: string;
  u_grid?: CurtainGrid;
  v_grid?: CurtainGrid;
  interior_mullion?: Profile;
  border_mullion?: Profile;
  panel?: CurtainPanel;
  panel_material?: string;
  mullion_material?: string;
}

export interface DeleteCurtainWallType {
  mutation: "deleteCurtainWallType";
  id: string;
}

export interface SetCurtainWallTypeOf {
  mutation: "setCurtainWallTypeOf";
  id: string;
  curtain_wall_type: string;
}

export interface SetCurtainWallGrid {
  mutation: "setCurtainWallGrid";
  id: string;
  u_grid?: CurtainGrid;
  v_grid?: CurtainGrid;
}

export interface CreateCurtainPanelOverride {
  mutation: "createCurtainPanelOverride";
  id: string;
  curtain_panel_override: CurtainPanelOverride;
}

export interface SetCurtainPanelOverride {
  mutation: "setCurtainPanelOverride";
  id: string;
  panel: CurtainPanel;
}

export interface DeleteCurtainPanelOverride {
  mutation: "deleteCurtainPanelOverride";
  id: string;
}

export interface CreateFamily {
  mutation: "createFamily";
  id: string;
  family: Family;
}

export interface DeleteFamily {
  mutation: "deleteFamily";
  id: string;
}

export interface SetFamily {
  mutation: "setFamily";
  id: string;
  name?: string;
  category?: FamilyCategory;
}

export interface SetFamilyParameter {
  mutation: "setFamilyParameter";
  family: string;
  name: string;
  kind?: ParameterKind;
  value?: string;
}

export interface RemoveFamilyParameter {
  mutation: "removeFamilyParameter";
  family: string;
  name: string;
}

export interface CreateFamilySolid {
  mutation: "createFamilySolid";
  id: string;
  solid: FamilySolid;
}

export interface DeleteFamilySolid {
  mutation: "deleteFamilySolid";
  id: string;
}

export interface SetFamilySolid {
  mutation: "setFamilySolid";
  id: string;
  name?: string;
  shape?: SolidShape;
  material?: string;
  visible?: string;
  offset?: ExprPoint3;
}

export type ModelMutation =
  | CreateSite
  | DeleteSite
  | CreateBuilding
  | DeleteBuilding
  | CreateStorey
  | RenameStorey
  | SetStoreyHeight
  | SetStoreyLevel
  | DeleteStorey
  | CreateWall
  | DeleteWall
  | SetWallTop
  | SetStoreyCutHeight
  | CreateMaterial
  | DeleteMaterial
  | SetMaterial
  | CreateWallType
  | DeleteWallType
  | SetWallType
  | CreateSlabType
  | DeleteSlabType
  | SetSlabType
  | CreateRoofType
  | DeleteRoofType
  | SetRoofType
  | CreateColumnType
  | DeleteColumnType
  | SetColumnType
  | CreateBeamType
  | DeleteBeamType
  | SetBeamType
  | CreateWindowType
  | DeleteWindowType
  | SetWindowType
  | CreateDoorType
  | DeleteDoorType
  | SetDoorType
  | SetProjectInfo
  | SetSite
  | SetBuilding
  | CreateGridLine
  | DeleteGridLine
  | SetGridLine
  | SetWallAxis
  | SetWallBaseOffset
  | SetWallTypeOf
  | SetWallLocation
  | FlipWall
  | SplitWall
  | CreateCurtainWall
  | DeleteCurtainWall
  | SetCurtainWall
  | CreateColumn
  | DeleteColumn
  | SetColumn
  | CreateBeam
  | DeleteBeam
  | SetBeam
  | CreateSlab
  | DeleteSlab
  | SetSlabBoundary
  | SetSlab
  | CreateRoof
  | DeleteRoof
  | SetRoofFootprint
  | SetRoofShape
  | CreateOpening
  | DeleteOpening
  | MoveOpening
  | SetOpening
  | CreateStair
  | DeleteStair
  | SetStair
  | CreateRailing
  | DeleteRailing
  | SetRailing
  | CreateSpace
  | DeleteSpace
  | SetSpace
  | MoveElements
  | RotateElements
  | DeleteElements
  | RenameElement
  | SetElementProperty
  | RemoveElementProperty
  | SetElementClassification
  | RemoveElementClassification
  | PlaceElements
  | SetElementStorey
  | SetElementPhase
  | SetWallEndJoin
  | CopyElements
  | MirrorElements
  | ArrayElements
  | AlignElements
  | OffsetWall
  | TrimExtendWall
  | SplitSlab
  | SplitBeam
  | CreateCeilingType
  | DeleteCeilingType
  | SetCeilingType
  | CreateCeiling
  | DeleteCeiling
  | SetCeilingBoundary
  | SetCeiling
  | CreateZone
  | SetZone
  | DeleteZone
  | CreateAreaScheme
  | SetAreaScheme
  | DeleteAreaScheme
  | CreateWallSweep
  | SetWallSweep
  | DeleteWallSweep
  | SetWallBaseSlab
  | CreateRamp
  | SetRamp
  | DeleteRamp
  | CreateDimension
  | DeleteDimension
  | SetDimension
  | CreateTag
  | DeleteTag
  | SetTag
  | CreateTextNote
  | DeleteTextNote
  | SetTextNote
  | CreateLeader
  | DeleteLeader
  | SetLeader
  | CreateAnnotationStyle
  | DeleteAnnotationStyle
  | SetAnnotationStyle
  | CreateView
  | SetView
  | DeleteView
  | CreateSchedule
  | SetSchedule
  | DeleteSchedule
  | CreateSheet
  | SetSheet
  | DeleteSheet
  | CreateViewport
  | SetViewport
  | DeleteViewport
  | CreateSheetRevision
  | SetSheetRevision
  | DeleteSheetRevision
  | CreatePropertyTemplate
  | SetPropertyTemplate
  | DeletePropertyTemplate
  | CreateClassificationSystem
  | SetClassificationSystem
  | DeleteClassificationSystem
  | SetBeamAxis
  | SetColumnTilt
  | CreateCurtainWallType
  | SetCurtainWallType
  | DeleteCurtainWallType
  | SetCurtainWallTypeOf
  | SetCurtainWallGrid
  | CreateCurtainPanelOverride
  | SetCurtainPanelOverride
  | DeleteCurtainPanelOverride
  | CreateFamily
  | DeleteFamily
  | SetFamily
  | SetFamilyParameter
  | RemoveFamilyParameter
  | CreateFamilySolid
  | DeleteFamilySolid
  | SetFamilySolid;
