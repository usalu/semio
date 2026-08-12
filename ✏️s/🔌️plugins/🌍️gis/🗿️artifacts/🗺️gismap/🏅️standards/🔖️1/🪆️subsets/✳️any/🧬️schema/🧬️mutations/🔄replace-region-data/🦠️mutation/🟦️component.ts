/** 🔁️ `replace-region-data` mutation payload — whole-value swaps a region feature's opaque payload (`MapFeature::data` is deliberately untyped, so a partial `change` isn't expressible — this is a `replace`, per the taxonomy's "large structured sub-payload" rule). */
export interface ReplaceRegionData {
  id: string;
  newData: unknown;
}

/** 🔖️ Semantic descriptor mirror: verb=`replace` entity=`region-data` kind=`replace-region-data` record=`ReplacedRegionData`. */
export const ReplaceRegionDataKind = "replace-region-data" as const;
