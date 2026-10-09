/** 🗺️ `plan-linework`: the architectural plan of a storey cut 1.2 m above its elevation as filled regions, stroked polylines (vertices carry a bulge) and text anchors, each with a style class and the id of its element. */

export type PlanStyle = "Cut" | "Projection" | "Hidden" | "Annotation";

export type PlanKind = "WallCut" | "WallLayer" | "WallOutline" | "CurtainAxis" | "CurtainMullion" | "WindowFrame" | "WindowGlazing" | "WindowSill" | "DoorLeaf" | "DoorSwing" | "ColumnCut" | "ColumnOutline" | "BeamOutline" | "SlabEdge" | "SlabHole" | "RoofOutline" | "StairOutline" | "StairRiser" | "StairCutLine" | "StairArrow" | "StairLanding" | "RailingPath" | "SpaceOutline" | "SpaceTag" | "GridLine" | "GridBubble" | "GridLabel" | "SectionCut" | "Silhouette" | "Edge" | "Datum" | "DatumLabel" | "CeilingEdge" | "CeilingHole" | "RampOutline" | "RampLanding" | "RampArrow" | "RampTag";

export interface PlanVertex {
  x: number;
  y: number;
  bulge: number;
}

export interface PlanRegion {
  id: string;
  element: string;
  kind: PlanKind;
  style: PlanStyle;
  outer: PlanVertex[];
  holes: PlanVertex[][];
}

export interface PlanPolyline {
  id: string;
  element: string;
  kind: PlanKind;
  style: PlanStyle;
  closed: boolean;
  vertices: PlanVertex[];
}

export interface PlanText {
  id: string;
  element: string;
  kind: PlanKind;
  style: PlanStyle;
  x: number;
  y: number;
  rotation: number;
  label: string;
  detail: string;
  measure?: number;
}

export interface PlanBounds {
  min_x: number;
  min_y: number;
  max_x: number;
  max_y: number;
}

export interface PlanLinework {
  storey: string;
  cut_height: number;
  cut_elevation: number;
  regions: PlanRegion[];
  polylines: PlanPolyline[];
  texts: PlanText[];
  bounds: PlanBounds;
}
