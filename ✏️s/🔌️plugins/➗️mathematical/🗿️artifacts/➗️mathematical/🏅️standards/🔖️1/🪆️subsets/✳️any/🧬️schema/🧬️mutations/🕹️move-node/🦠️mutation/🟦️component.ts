/** 🕹️ `move-node` — absolute spatial reposition of a graph node (the node-graph canvas's `move` edit op). */
export interface MoveNode {
  id: string;
  x: number;
  y: number;
}

/** 🔖️ Semantic descriptor mirror: verb=`move` entity=`node` kind=`move-node` record=`MovedNode`. */
export const MoveNodeKind = "move-node" as const;
