/** 🔹 `delete-tiles` mutation payload — removes multiple figure tile crops from tiles by id (multi-select). */
export interface DeleteTiles {
  ids: string[];
}

/** 🔖️ Semantic descriptor mirror: verb=`delete` entity=`tiles` kind=`delete-tiles` record=`DeletedTiles`. */
export const DeleteTilesKind = "delete-tiles" as const;
