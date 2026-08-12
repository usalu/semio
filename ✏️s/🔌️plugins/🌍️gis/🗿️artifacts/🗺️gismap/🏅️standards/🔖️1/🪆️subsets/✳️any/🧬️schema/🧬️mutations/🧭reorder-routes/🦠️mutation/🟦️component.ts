/** 🔀️ `reorder-routes` mutation payload — repositions a route feature within `routes` by id (id-keyed collection, so addressing is `id`+`to_index`, not a bare index pair). */
export interface ReorderRoutes {
  id: string;
  toIndex: number;
}

/** 🔖️ Semantic descriptor mirror: verb=`reorder` entity=`routes` kind=`reorder-routes` record=`ReorderedRoutes`. */
export const ReorderRoutesKind = "reorder-routes" as const;
