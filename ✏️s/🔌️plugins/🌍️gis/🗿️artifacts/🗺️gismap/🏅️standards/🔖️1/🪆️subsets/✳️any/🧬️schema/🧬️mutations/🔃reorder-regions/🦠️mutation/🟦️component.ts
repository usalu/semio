/** 🔀️ `reorder-regions` mutation payload — repositions a region feature within `regions` by id (id-keyed collection, so addressing is `id`+`to_index`, not a bare index pair). */
export interface ReorderRegions {
  id: string;
  toIndex: number;
}

/** 🔖️ Semantic descriptor mirror: verb=`reorder` entity=`regions` kind=`reorder-regions` record=`ReorderedRegions`. */
export const ReorderRegionsKind = "reorder-regions" as const;
