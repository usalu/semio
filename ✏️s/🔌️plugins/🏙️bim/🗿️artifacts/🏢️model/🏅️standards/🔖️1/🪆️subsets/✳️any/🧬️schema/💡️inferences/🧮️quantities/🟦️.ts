/** 🧮️ `quantities`: the quantity take-off per element and the totals per kind, type and material for every storey, building and the project. */

export interface LayerQuantity {
  material: string;
  thickness: number;
  area: number;
  volume: number;
  mass: number;
}

export type QuantityKind = "Wall" | "CurtainWall" | "Slab" | "Roof" | "Column" | "Beam" | "Window" | "Door" | "Void" | "Stair" | "Railing" | "Space";

export interface ElementQuantity {
  kind: QuantityKind;
  storey: string;
  type_id: string;
  count: number;
  length: number;
  width: number;
  height: number;
  perimeter: number;
  gross_side_area: number;
  opening_area: number;
  net_side_area: number;
  gross_area: number;
  net_area: number;
  surface_area: number;
  gross_volume: number;
  net_volume: number;
  mass: number;
  risers: number;
  layers: LayerQuantity[];
}

export interface Totals {
  count: number;
  length: number;
  area: number;
  volume: number;
  mass: number;
}

export interface QuantityTotals {
  kinds: Record<string, Totals>;
  types: Record<string, Totals>;
  materials: Record<string, Totals>;
}

export interface ModelQuantities {
  elements: Record<string, ElementQuantity>;
  storeys: Record<string, QuantityTotals>;
  buildings: Record<string, QuantityTotals>;
  project: QuantityTotals;
}
