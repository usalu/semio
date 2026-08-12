/** 🔹 `replace-tiles` mutation payload — whole-value swaps the tiles collection. */
export interface ReplaceTiles {
  newTiles: unknown[];
}

/** 🔖️ Semantic descriptor mirror: verb=`replace` entity=`tiles` kind=`replace-tiles` record=`ReplacedTiles`. */
export const ReplaceTilesKind = "replace-tiles" as const;
