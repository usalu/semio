/** 🔹 `replace-stock-solid` mutation payload — whole-value swaps the stock's solid geometry. */
export interface ReplaceStockSolid {
  newSolid: unknown;
}

/** 🔖️ Semantic descriptor mirror: verb=`replace` entity=`stock` kind=`replace-stock-solid` record=`ReplacedStockSolid`. */
export const ReplaceStockSolidKind = "replace-stock-solid" as const;
