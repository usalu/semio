/** 🪟️ `opening-frames`: the resolved placement of every window, door and void in its host: size and sill, frames, cut rectangle in the host's (s, z) development, reveal depth, plan strokes and validity. */

export interface Point2 {
  x: number;
  y: number;
}

export interface Vec3 {
  x: number;
  y: number;
  z: number;
}

export interface Frame {
  origin: Vec3;
  x_axis: Vec3;
  y_axis: Vec3;
  z_axis: Vec3;
}

export interface OpeningCut {
  s_min: number;
  s_max: number;
  z_min: number;
  z_max: number;
}

export type OpeningIssue = "HostMissing" | "TypeMissing" | "NonPositiveSize" | "OutsideHostExtent" | "BelowHostBase" | "AboveHostTop" | "OverlapsSibling";

export type PlanRole = "Leaf" | "Swing" | "Glazing";

export type PlanShape =
  | { Line: { from: Point2; to: Point2 } }
  | { Arc: { centre: Point2; radius: number; start_angle: number; sweep: number } };

export interface PlanStroke {
  role: PlanRole;
  shape: PlanShape;
}

export type Swing = "Left" | "Right";

export interface OpeningFrame {
  width: number;
  height: number;
  sill: number;
  offset: number;
  cut: OpeningCut;
  reveal_depth: number;
  face_front: number;
  face_back: number;
  host_length: number;
  host_height: number;
  point: Point2;
  local: Frame;
  world: Frame;
  hand?: Swing;
  plan: PlanStroke[];
  issues: OpeningIssue[];
  overlaps: string[];
  valid: boolean;
}
