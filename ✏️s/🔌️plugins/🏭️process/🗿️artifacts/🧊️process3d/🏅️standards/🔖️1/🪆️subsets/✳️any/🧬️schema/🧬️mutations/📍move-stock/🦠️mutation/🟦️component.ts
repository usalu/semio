/** 🔹 `move-stock` mutation payload — repositions the stock. */
export interface MoveStock {
  newPose: unknown;
}

/** 🔖️ Semantic descriptor mirror: verb=`move` entity=`stock` kind=`move-stock` record=`MovedStock`. */
export const MoveStockKind = "move-stock" as const;
