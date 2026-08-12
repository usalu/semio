/** 🔹 `create-node` mutation payload — adds a new board node. */
export interface CreateNode {
  node: unknown;
}

/** 🔖️ Semantic descriptor mirror: verb=`create` entity=`node` kind=`create-node` record=`CreatedNode`. */
export const CreateNodeKind = "create-node" as const;
