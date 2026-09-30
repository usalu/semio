/** 🧬️ MoveNode payload owned by the move-node mutation. */
export interface MoveNode {
  mutation: "moveNode";
  nodeId: string;
  newX: number;
  newY: number;
}
