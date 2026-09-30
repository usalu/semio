/** 🧬️ ResizeNode payload owned by the resize-node mutation. */
export interface ResizeNode {
  mutation: "resizeNode";
  nodeId: string;
  newRadius?: number;
  newWidth?: number;
  newHeight?: number;
}
