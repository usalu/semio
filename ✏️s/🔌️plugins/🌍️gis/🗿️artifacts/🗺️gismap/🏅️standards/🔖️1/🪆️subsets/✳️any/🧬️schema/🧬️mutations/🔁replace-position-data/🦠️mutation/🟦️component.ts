/** 🔁️ `replace-position-data` mutation payload — whole-value swaps a position feature's opaque payload (`MapFeature::data` is deliberately untyped, so a partial `change` isn't expressible — this is a `replace`, per the taxonomy's "large structured sub-payload" rule). */
export interface ReplacePositionData {
  id: string;
  newData: unknown;
}

/** 🔖️ Semantic descriptor mirror: verb=`replace` entity=`position-data` kind=`replace-position-data` record=`ReplacedPositionData`. */
export const ReplacePositionDataKind = "replace-position-data" as const;
