/** 🏠️ `spaces`: the resolved room of a space: outline and islands, areas, clear height, volume and the bounding walls, or the reason the seed does not bound a room. */

export interface Point2 {
  x: number;
  y: number;
}

export interface Vertex {
  point: Point2;
  bulge: number;
}

export type SpaceStatus = "Inferred" | "Explicit" | "NotEnclosed" | "SeedInsideWall" | "InvalidOutline";

export interface SpaceRoom {
  status: SpaceStatus;
  outline: Vertex[];
  holes: Vertex[][];
  point: Point2;
  area: number;
  perimeter: number;
  net_floor_area: number;
  floor_z: number;
  clear_height: number;
  volume: number;
  ceiling_slab: string;
  bounding_walls: string[];
}
