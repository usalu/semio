/** 🧱️ `wall-layout`: the resolved vertical extent, plan geometry (faces, join-trimmed footprint loop, joins) and quantities of a wall. Layers run from the left (interior) face to the right (exterior) face along the axis. */

export type JoinEnd = "Start" | "End" | "Along";

export type JoinKind = "Miter" | "Butt" | "Through" | "Cross";

export interface Point2 {
  x: number;
  y: number;
}

export interface Vertex {
  point: Point2;
  bulge: number;
}

export interface FaceCurve {
  start: Point2;
  end: Point2;
  bulge: number;
}

export interface WallJoin {
  kind: JoinKind;
  end: JoinEnd;
  other: string;
  other_end: JoinEnd;
  point: Point2;
  overlap_area: number;
}

export interface WallLayout {
  base_z: number;
  top_z: number;
  height: number;
  thickness: number;
  length: number;
  offset_left: number;
  offset_right: number;
  layer_offsets: number[];
  left_face: FaceCurve;
  right_face: FaceCurve;
  left_length: number;
  right_length: number;
  left_area: number;
  right_area: number;
  side_area: number;
  footprint: Vertex[];
  footprint_area: number;
  volume: number;
  joins: WallJoin[];
}
