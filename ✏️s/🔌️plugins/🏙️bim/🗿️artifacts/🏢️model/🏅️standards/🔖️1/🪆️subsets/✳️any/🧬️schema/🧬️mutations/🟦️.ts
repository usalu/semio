/** 🏙️ BIM model direct-mutation discriminated union. */
import type { Axis, Baluster, Beam, BeamType, Building, Classification, Column, ColumnType, CurtainWall, DoorLeaves, DoorType, GridLine, Infill, Layer, LocationLine, Material, MaterialCategory, Opening, OpeningKind, Point2, Profile, PropertyValue, Railing, Rgb, RiserKind, Roof, RoofShape, RoofType, Site, Slab, SlabType, Slope, Space, SpaceBoundary, Stair, StairFlight, StairStringer, Storey, Swing, TopConstraint, Vertex, Wall, WallType, WindowType } from "../🟦️.ts";

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
  u_spacing?: number;
  v_spacing?: number;
  mullion?: Profile;
  panel_material?: string;
  mullion_material?: string;
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
  start?: Point2;
  end?: Point2;
  top_offset?: number;
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
  classification: Classification;
}

export interface RemoveElementClassification {
  mutation: "removeElementClassification";
  id: string;
}

export interface PlaceElements {
  mutation: "placeElements";
  placements: Record<string, unknown>;
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
  | PlaceElements;
