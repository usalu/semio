/** 🔹 `change-stock-label` mutation payload — sets the stock's display label. */
export interface ChangeStockLabel {
  newLabel: string;
}

/** 🔖️ Semantic descriptor mirror: verb=`change` entity=`stock` kind=`change-stock-label` record=`ChangedStockLabel`. */
export const ChangeStockLabelKind = "change-stock-label" as const;
