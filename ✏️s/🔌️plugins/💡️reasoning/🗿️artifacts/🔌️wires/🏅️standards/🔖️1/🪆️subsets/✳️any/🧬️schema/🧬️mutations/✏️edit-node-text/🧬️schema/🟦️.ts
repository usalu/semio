/** 🧬️ EditNodeText payload owned by the edit-node-text mutation. */
export interface EditNodeText {
  mutation: "editNodeText";
  nodeId: string;
  newText: string;
}
