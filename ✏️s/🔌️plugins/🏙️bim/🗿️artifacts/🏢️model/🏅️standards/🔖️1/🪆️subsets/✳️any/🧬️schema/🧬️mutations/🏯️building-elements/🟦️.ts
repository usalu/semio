/** 🏯️ BIM model direct mutations for walls, curtain walls, columns, beams, slabs, roofs, ceilings, openings, stairs, railings, ramps, supports and loads. */
import type { Axis, Baluster, Beam, Ceiling, Column, CurtainGrid, CurtainPanel, CurtainPanelOverride, CurtainWall, EndJoin, Infill, LocationLine, Opening, OpeningKind, Point2, Point3, Profile, Railing, RailingHost, Ramp, RiserKind, Roof, RoofShape, Slab, Slope, Stair, StairFlight, StairStringer, TopConstraint, Vertex, Wall, WallSide, WallSweep, StructuralSupport, LoadCase, StructuralLoad, StructuralLocation, Restraints } from "../../🟦️.ts";

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

export interface SetWallEndJoin {
  mutation: "setWallEndJoin";
  id: string;
  end: string;
  join?: EndJoin;
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
export interface CreateSupport { mutation: "createSupport"; id: string; support: StructuralSupport; }
export interface SetSupport { mutation: "setSupport"; id: string; name?: string; member?: string; location?: StructuralLocation; offset?: Point3; restraints?: Restraints; }
export interface DeleteSupport { mutation: "deleteSupport"; id: string;  }
export interface CreateLoadCase { mutation: "createLoadCase"; id: string; load_case: LoadCase; }
export interface SetLoadCase { mutation: "setLoadCase"; id: string; name?: string; category?: string; factor?: number; }
export interface DeleteLoadCase { mutation: "deleteLoadCase"; id: string;  }
export interface CreateLoad { mutation: "createLoad"; id: string; load: StructuralLoad; }
export interface SetLoad { mutation: "setLoad"; id: string; name?: string; load_case?: string; member?: string; location?: StructuralLocation; force?: Point3; moment?: Point3; }
export interface DeleteLoad { mutation: "deleteLoad"; id: string;  }
