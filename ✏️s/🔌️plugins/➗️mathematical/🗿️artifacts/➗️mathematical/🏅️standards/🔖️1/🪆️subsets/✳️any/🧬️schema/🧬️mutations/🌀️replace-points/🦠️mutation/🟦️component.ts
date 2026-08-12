/** 🌀️ `replace-points` — whole-value swap of the geometry playground's point cloud — the semantic replacement for the old generic `SetGeometry`, used by gestures that load/paste an entire point set (the app's `SetPoints` command) rather than editing one point. */
export interface ReplacePoints {
  points: (unknown)[];
}

/** 🔖️ Semantic descriptor mirror: verb=`replace` entity=`points` kind=`replace-points` record=`ReplacedPoints`. */
export const ReplacePointsKind = "replace-points" as const;
