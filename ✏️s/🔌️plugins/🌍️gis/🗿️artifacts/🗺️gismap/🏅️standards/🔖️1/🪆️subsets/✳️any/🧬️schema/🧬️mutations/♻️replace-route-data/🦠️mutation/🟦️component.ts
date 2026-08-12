/** 🔁️ `replace-route-data` mutation payload — whole-value swaps a route feature's opaque payload (`MapFeature::data` is deliberately untyped, so a partial `change` isn't expressible — this is a `replace`, per the taxonomy's "large structured sub-payload" rule). */
export interface ReplaceRouteData {
  id: string;
  newData: unknown;
}

/** 🔖️ Semantic descriptor mirror: verb=`replace` entity=`route-data` kind=`replace-route-data` record=`ReplacedRouteData`. */
export const ReplaceRouteDataKind = "replace-route-data" as const;
