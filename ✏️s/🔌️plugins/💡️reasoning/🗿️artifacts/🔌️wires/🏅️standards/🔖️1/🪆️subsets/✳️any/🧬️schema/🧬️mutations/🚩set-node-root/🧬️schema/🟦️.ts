/** 🧬️ SetNodeRoot payload owned by the set-node-root mutation. */
export interface SetNodeRoot {
  mutation: "setNodeRoot";
  nodeId: string;
  newRoot: boolean;
}
