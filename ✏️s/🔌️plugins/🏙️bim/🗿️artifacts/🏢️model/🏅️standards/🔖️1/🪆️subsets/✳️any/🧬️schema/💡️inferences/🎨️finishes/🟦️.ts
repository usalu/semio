/** 🎨️ `finishes`: the floor, wall and ceiling finish areas of a room: the material the space names (empty when unfinished) and the area it covers, per surface. */

export type FinishSurface = "Floor" | "Wall" | "Ceiling";

export interface FinishQuantity {
  surface: FinishSurface;
  material: string;
  area: number;
}
