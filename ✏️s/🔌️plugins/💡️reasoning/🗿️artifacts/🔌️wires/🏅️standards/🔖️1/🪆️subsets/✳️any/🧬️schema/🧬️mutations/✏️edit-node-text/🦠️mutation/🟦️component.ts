/** 🔹 `edit-node-text` mutation payload — replaces a board node's authored text. */
export interface EditNodeText {
  nodeId: string;
  newText: string;
}

/** 🔖️ Semantic descriptor mirror: verb=`edit` entity=`node` kind=`edit-node-text` record=`EditedNodeText`. */
export const EditNodeTextKind = "edit-node-text" as const;
