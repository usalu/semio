/** 🧬️ BIM model snapshot schema — authored parameters only; nothing derivable is stored. */

import type { PropertyKind, TemplateTarget, AnchorEnd, WallSide, TagCategory, Terminator, DimensionUnit, FamilyCategory, ParameterKind, SolidAxis, IsoSize, Orientation, MaterialCategory, LayerFunction, LocationLine, Phase, DoorLeaves, Swing, Turn, StringerKind, RiserKind, ScheduleCategory, ScheduleField, ScheduleOp, EndJoin, ViewKind, DetailLevel, ViewCategory, HostSide, AreaMeasure, AnnotationAnchor, ParametricProfile, SolidShape, Paper, CurtainGrid, CurtainPanel, Axis, TopConstraint, Profile, RoofShape, OpeningKind, StairFlight, PropertyValue, Infill, SpaceBoundary, ScheduleKey, PropertyDef, PropertyTemplate, ClassificationItem, ClassificationSystem, Dimension, Tag, TextNote, Leader, AnnotationStyle, ExprPoint, ExprPoint3, Family, FamilyParameter, FamilySolid, Sheet, Viewport, SheetRevision, WallSweep, Point2, Rgb, Vertex, Slope, StairStringer, Baluster, RailingHost, Layer, Project, Material, WallType, SlabType, CeilingType, RoofType, ColumnType, BeamType, WindowType, DoorType, Site, Building, Storey, GridLine, Wall, CurtainWallType, CurtainPanelOverride, CurtainWall, Column, Beam, Slab, Ceiling, Roof, Opening, Stair, Railing, Ramp, Space, Zone, AreaScheme, ViewPlane, ViewCrop, ViewCamera, View, ScheduleColumn, ScheduleSort, ScheduleFilter, ScheduleGroup, Schedule, PropertySet, ClassificationSet } from "../🟦️.ts";

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

