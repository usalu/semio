/** 💰️ The inferred quantity billed by one authored cost item, in SI units. */
export type CostBasis = "Count" | "Length" | "GrossArea" | "NetArea" | "SurfaceArea" | "GrossSideArea" | "NetSideArea" | "GrossVolume" | "NetVolume" | "Mass";

/** 🧾️ Authored pricing with an explicit currency label and line rounding precision. */
export interface CostItem {
  name: string;
  currency: string;
  basis: CostBasis;
  unit_cost: number;
  precision: number;
}

/** 🔗️ An independently addressable type-to-item link; factor accounts for waste or multiple applications. */
export interface TypeCostLink {
  type_id: string;
  item: string;
  factor: number;
}
