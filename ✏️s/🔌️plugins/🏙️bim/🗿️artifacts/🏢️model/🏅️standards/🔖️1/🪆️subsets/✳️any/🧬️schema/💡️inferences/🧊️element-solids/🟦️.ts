/** 🧊️ `element-solids`: the owned triangle mesh of every building element, keyed by element id, in building-local metres; `placement` is the instance transform into the world. */

export type SolidFamily = "Wall" | "CurtainWall" | "Window" | "Door" | "Column" | "Beam" | "Slab" | "Roof" | "Stair" | "Ramp" | "Railing" | "Ceiling";

export interface SolidPoint {
  x: number;
  y: number;
  z: number;
}

export interface SolidBounds {
  min: SolidPoint;
  max: SolidPoint;
}

export interface SolidPlacement {
  x: number;
  y: number;
  z: number;
  rotation: number;
}

/** A run of faces that share a part (`layer`, `panel`, `mullion`, `frame`, `muntin`, `glass`, `leaf`, ...), a material id and a layer index. */
export interface SolidGroup {
  part: string;
  material: string;
  layer: number;
}

/** Flat `positions` and `normals` (xyz per vertex, one vertex triple per triangle), flat `indices` (3 per triangle) and one `face_groups` entry per triangle. */
export interface ElementSolid {
  family: SolidFamily;
  storey: string;
  placement: SolidPlacement;
  groups: SolidGroup[];
  positions: number[];
  normals: number[];
  indices: number[];
  face_groups: number[];
  bounds: SolidBounds;
  volume: number;
  area: number;
}
