/** 🔹 `move-node` mutation payload — repositions a board node. */
export interface MoveNode {
  nodeId: string;
  newX: number;
  newY: number;
}

/** 🔖️ Semantic descriptor mirror: verb=`move` entity=`node` kind=`move-node` record=`MovedNode`. */
export const MoveNodeKind = "move-node" as const;
