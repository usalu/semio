/** 🧬️ BIM model snapshot schema — authored parameters only; nothing derivable is stored. */

import type { MaterialCategory, LayerFunction, LocationLine, Phase, DoorLeaves, Swing, Turn, StringerKind, RiserKind, Axis, TopConstraint, Profile, RoofShape, OpeningKind, StairFlight, PropertyValue, Infill, SpaceBoundary, Point2, Rgb, Vertex, Slope, StairStringer, Baluster, Layer, Project, Material, WallType, SlabType, RoofType, ColumnType, BeamType, WindowType, DoorType, Site, Building, Storey, GridLine, Wall, CurtainWall, Column, Beam, Slab, Roof, Opening, Stair, Railing, Space, Classification, PropertySet } from "../🟦️.ts";

export interface ModelSnapshot {
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

