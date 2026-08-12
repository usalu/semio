/** 🔹 `change-node-shape` mutation payload — sets a board node's shape. */
export interface ChangeNodeShape {
  nodeId: string;
  newShape: string;
}

/** 🔖️ Semantic descriptor mirror: verb=`change` entity=`node` kind=`change-node-shape` record=`ChangedNodeShape`. */
export const ChangeNodeShapeKind = "change-node-shape" as const;
