/** ❌️ `delete-node` — removes an id-keyed graph node, cascading to every edge incident on it. */
export interface DeleteNode {
  id: string;
}

/** 🔖️ Semantic descriptor mirror: verb=`delete` entity=`node` kind=`delete-node` record=`DeletedNode`. */
export const DeleteNodeKind = "delete-node" as const;
