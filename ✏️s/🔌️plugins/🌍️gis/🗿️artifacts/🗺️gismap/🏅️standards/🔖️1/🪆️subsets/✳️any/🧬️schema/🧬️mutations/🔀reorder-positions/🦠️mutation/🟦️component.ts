/** 🔀️ `reorder-positions` mutation payload — repositions a position feature within `positions` by id (id-keyed collection, so addressing is `id`+`to_index`, not a bare index pair). */
export interface ReorderPositions {
  id: string;
  toIndex: number;
}

/** 🔖️ Semantic descriptor mirror: verb=`reorder` entity=`positions` kind=`reorder-positions` record=`ReorderedPositions`. */
export const ReorderPositionsKind = "reorder-positions" as const;
