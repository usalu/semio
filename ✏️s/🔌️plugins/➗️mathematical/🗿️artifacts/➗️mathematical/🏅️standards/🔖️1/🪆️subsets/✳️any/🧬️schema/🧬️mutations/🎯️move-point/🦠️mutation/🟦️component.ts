/** 🎯️ `move-point` — absolute spatial reposition of a point in the geometry playground's point cloud, addressed by its BASE-state index. */
export interface MovePoint {
  index: number;
  x: number;
  y: number;
}

/** 🔖️ Semantic descriptor mirror: verb=`move` entity=`point` kind=`move-point` record=`MovedPoint`. */
export const MovePointKind = "move-point" as const;
