/** 🧩️ Wires direct-mutation discriminated union. */

export type DslValue = Record<string, unknown>;

/** 🌱 `create-node` payload — the node's full initial state. */
export interface CreateNode {
  node: DslValue;
}

/** 🗑️ `delete-node` payload — the node's id. */
export interface DeleteNode {
  nodeId: string;
}

/** 🧭️ `move-node` payload — the node's new absolute board position. */
export interface MoveNode {
  nodeId: string;
  newX: number;
  newY: number;
}

/** 📐️ `resize-node` payload — only the extent fields actually being changed are set. */
export interface ResizeNode {
  nodeId: string;
  newRadius?: number;
  newWidth?: number;
  newHeight?: number;
}

/** 🏷️ `change-node-kind` payload. */
export interface ChangeNodeKind {
  nodeId: string;
  newNodeKind: string;
}

/** 🔷 `change-node-shape` payload. */
export interface ChangeNodeShape {
  nodeId: string;
  newShape: string;
}

/** ✏️ `edit-node-text` payload. */
export interface EditNodeText {
  nodeId: string;
  newText: string;
}

/** 🚩 `set-node-root` payload. */
export interface SetNodeRoot {
  nodeId: string;
  newRoot: boolean;
}

/** 🔗 `connect-nodes` payload — the full new board edge, plus its (possibly null) relationship. */
export interface ConnectNodes {
  edge: DslValue;
  relationship: DslValue;
}

/** ✂️ `disconnect-nodes` payload — the edge's id. */
export interface DisconnectNodes {
  edgeId: string;
}

export type WiresMutation =
  | ({ mutation: "createNode" } & CreateNode)
  | ({ mutation: "deleteNode" } & DeleteNode)
  | ({ mutation: "moveNode" } & MoveNode)
  | ({ mutation: "resizeNode" } & ResizeNode)
  | ({ mutation: "changeNodeKind" } & ChangeNodeKind)
  | ({ mutation: "changeNodeShape" } & ChangeNodeShape)
  | ({ mutation: "editNodeText" } & EditNodeText)
  | ({ mutation: "setNodeRoot" } & SetNodeRoot)
  | ({ mutation: "connectNodes" } & ConnectNodes)
  | ({ mutation: "disconnectNodes" } & DisconnectNodes);
