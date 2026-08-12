/** 🔹 `change-node-kind` mutation payload — sets a board node's kind. */
export interface ChangeNodeKind {
  nodeId: string;
  newNodeKind: string;
}

/** 🔖️ Semantic descriptor mirror: verb=`change` entity=`node` kind=`change-node-kind` record=`ChangedNodeKind`. */
export const ChangeNodeKindKind = "change-node-kind" as const;
