/** 🔹 `reorder-tiles` mutation payload — repositions a figure tile within tiles by id. */
export interface ReorderTiles {
  id: string;
  toIndex: number;
}

/** 🔖️ Semantic descriptor mirror: verb=`reorder` entity=`tiles` kind=`reorder-tiles` record=`ReorderedTiles`. */
export const ReorderTilesKind = "reorder-tiles" as const;
