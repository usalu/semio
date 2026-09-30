/** 🧬️ ChangeNodeKind payload owned by the change-node-kind mutation. */
export interface ChangeNodeKind {
  mutation: "changeNodeKind";
  nodeId: string;
  newNodeKind: string;
}
