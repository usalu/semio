/** 🗑️ `delete-nodes` — plural/bulk delete, the real multi-select gesture behind the node-graph canvas's `deleteSelection` edit op (`✏️editor/🎮️commands/🕸️set-algorithm/component.rs`) — a separate mutation per taxonomy's "Bulk/plural mutations" rule, never a bare `Vec` bolted onto the singular `delete-node`. */
export interface DeleteNodes {
  ids: (string)[];
}

/** 🔖️ Semantic descriptor mirror: verb=`delete` entity=`nodes` kind=`delete-nodes` record=`DeletedNodes`. */
export const DeleteNodesKind = "delete-nodes" as const;
