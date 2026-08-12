/** ➖️ `remove-point` — takes a point out of the geometry playground's point cloud. `index` is BASE-state, per the addressing convention for index-keyed collections. */
export interface RemovePoint {
  index: number;
}

/** 🔖️ Semantic descriptor mirror: verb=`remove` entity=`point` kind=`remove-point` record=`RemovedPoint`. */
export const RemovePointKind = "remove-point" as const;
