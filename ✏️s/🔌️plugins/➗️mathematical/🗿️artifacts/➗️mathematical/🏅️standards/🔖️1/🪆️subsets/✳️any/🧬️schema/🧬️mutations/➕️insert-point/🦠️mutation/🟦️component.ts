/** ➕️ `insert-point` — places a new point into the geometry playground's anonymous, index-keyed point cloud. `index` is FINAL-state, per the addressing convention for index-keyed collections. */
export interface InsertPoint {
  index: number;
  x: number;
  y: number;
}

/** 🔖️ Semantic descriptor mirror: verb=`insert` entity=`point` kind=`insert-point` record=`InsertedPoint`. */
export const InsertPointKind = "insert-point" as const;
