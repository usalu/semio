/** 🧬️ ChangeNodeShape payload owned by the change-node-shape mutation. */
export interface ChangeNodeShape {
  mutation: "changeNodeShape";
  nodeId: string;
  newShape: string;
}
