/** 🔹 `resize-node` mutation payload — changes a board node's extent (only the fields actually being changed are set). */
export interface ResizeNode {
  nodeId: string;
  newRadius?: number;
  newWidth?: number;
  newHeight?: number;
}

/** 🔖️ Semantic descriptor mirror: verb=`resize` entity=`node` kind=`resize-node` record=`ResizedNode`. */
export const ResizeNodeKind = "resize-node" as const;
