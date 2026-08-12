/** 🧮️ `update-graph-algorithm` — the algorithm id and its seed are validated together (the seed is only meaningful in the context of the algorithm it seeds), so this is the recipe's inseparable-facet `update` exception rather than two independent `change-` scalars — matches the app's `SetAlgorithm` command, which always sends both fields together. */
export interface UpdateGraphAlgorithm {
  newAlgorithm: string;
  newAlgorithmSeed: string | null;
}

/** 🔖️ Semantic descriptor mirror: verb=`update` entity=`graph` kind=`update-graph-algorithm` record=`UpdatedGraphAlgorithm`. */
export const UpdateGraphAlgorithmKind = "update-graph-algorithm" as const;
