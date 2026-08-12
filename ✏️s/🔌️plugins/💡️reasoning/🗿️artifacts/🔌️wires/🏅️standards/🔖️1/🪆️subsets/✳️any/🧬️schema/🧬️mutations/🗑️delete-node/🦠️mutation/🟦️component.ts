/** 🔹 `delete-node` mutation payload — removes a board node by id. */
export interface DeleteNode {
  nodeId: string;
}

/** 🔖️ Semantic descriptor mirror: verb=`delete` entity=`node` kind=`delete-node` record=`DeletedNode`. */
export const DeleteNodeKind = "delete-node" as const;
