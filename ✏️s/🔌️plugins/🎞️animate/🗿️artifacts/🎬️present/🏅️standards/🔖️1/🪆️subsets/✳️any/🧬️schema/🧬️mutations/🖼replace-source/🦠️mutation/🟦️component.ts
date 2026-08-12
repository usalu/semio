/** 🔹 `replace-source` mutation payload — whole-value swaps the shared figure source. */
export interface ReplaceSource {
  newSource: unknown;
}

/** 🔖️ Semantic descriptor mirror: verb=`replace` entity=`source` kind=`replace-source` record=`ReplacedSource`. */
export const ReplaceSourceKind = "replace-source" as const;
