/** 🔹 `set-node-root` mutation payload — sets whether a board node is the tree root. */
export interface SetNodeRoot {
  nodeId: string;
  newRoot: boolean;
}

/** 🔖️ Semantic descriptor mirror: verb=`set` entity=`node` kind=`set-node-root` record=`SetNodeRoot`. */
export const SetNodeRootKind = "set-node-root" as const;
