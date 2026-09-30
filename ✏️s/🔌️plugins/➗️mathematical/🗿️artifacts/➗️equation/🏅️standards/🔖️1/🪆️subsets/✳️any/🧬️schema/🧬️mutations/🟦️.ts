/** ➗️ EquationMutation — closed semantic mutation vocabulary for the equation document,
 *  mirrors `🧬️mutations/🦀️.rs`'s `EquationMutation` enum and its 15 per-verb leaf structs. The enum
 *  carries no `#[value(tag)]`, so it wires EXTERNALLY TAGGED: `{ "<PascalCaseVariantName>": { ...leaf
 *  fields } }`. Every leaf struct carries `#[value(rename_all = "camelCase")]`, so its fields wire
 *  camelCase (`{"ChangeNodeLabel":{"id":"n-alpha","newLabel":"Alpha"}}`), like the referenced
 *  `EquationGraph`/`EquationPoint` records (`ReplaceGraph.graph.algorithmSeed`). */
/** 🔵️ One graph-playground node — mirrors the artifact's `EquationNode` record (`rename_all = "camelCase"`). */
export interface EquationNode {
  id: string;
  label: string;
  x: number;
  y: number;
}

/** 🔌️ One graph-playground edge — mirrors `EquationEdge`. */
export interface EquationEdge {
  id: string;
  source: string;
  target: string;
}

/** 🕸️ The graph playground — mirrors `EquationGraph`; `algorithmSeed` wires `null` when unset. */
export interface EquationGraph {
  directed: boolean;
  nodes: EquationNode[];
  edges: EquationEdge[];
  algorithm: string;
  algorithmSeed: string | null;
}

/** 📍️ One geometry-playground point — mirrors `EquationPoint`. */
export interface EquationPoint {
  x: number;
  y: number;
}

/** 🔢️ Addresses one numeric leaf in the equation tree — a `u64` newtype, wire-plain as a number. */
export type EquationNodeLabel = number;

/** 🔀️ `change-graph-directed` payload. */
export interface ChangeGraphDirected {
  newDirected: boolean;
}

/** 🧮️ `update-graph-algorithm` payload — algorithm id and seed are validated together. */
export interface UpdateGraphAlgorithm {
  newAlgorithm: string;
  newAlgorithmSeed: string | null;
}

/** 🔁️ `replace-graph` payload — whole-value swap of the graph playground's structured payload. */
export interface ReplaceGraph {
  graph: EquationGraph;
}

/** 🟢️ `create-node` payload. */
export interface CreateNode {
  id: string;
  label: string;
  x: number;
  y: number;
  index?: number;
}

/** ❌️ `delete-node` payload. */
export interface DeleteNode {
  id: string;
}

/** 🗑️ `delete-nodes` payload — bulk delete by id. */
export interface DeleteNodes {
  ids: string[];
}

/** 🏷️ `change-node-label` payload. */
export interface ChangeNodeLabel {
  id: string;
  newLabel: string;
}

/** 🕹️ `move-node` payload. */
export interface MoveNode {
  id: string;
  x: number;
  y: number;
}

/** 🔗️ `connect-nodes` payload. */
export interface ConnectNodes {
  id: string;
  source: string;
  target: string;
  index?: number;
}

/** ✂️ `disconnect-nodes` payload. */
export interface DisconnectNodes {
  id: string;
}

/** 🌀️ `replace-points` payload — whole-value swap of the geometry playground's point cloud. */
export interface ReplacePoints {
  points: EquationPoint[];
}

/** ➕️ `insert-point` payload. */
export interface InsertPoint {
  index: number;
  x: number;
  y: number;
}

/** ➖️ `remove-point` payload. */
export interface RemovePoint {
  index: number;
}

/** 🎯️ `move-point` payload. */
export interface MovePoint {
  index: number;
  x: number;
  y: number;
}

/** 🔄️ `change-coefficient` payload — sets a numeric leaf's value in the equation tree. */
export interface ChangeCoefficient {
  label: EquationNodeLabel;
  numer: string;
  denom: string;
}

export type EquationMutation =
  | { ChangeGraphDirected: ChangeGraphDirected }
  | { UpdateGraphAlgorithm: UpdateGraphAlgorithm }
  | { ReplaceGraph: ReplaceGraph }
  | { CreateNode: CreateNode }
  | { DeleteNode: DeleteNode }
  | { DeleteNodes: DeleteNodes }
  | { ChangeNodeLabel: ChangeNodeLabel }
  | { MoveNode: MoveNode }
  | { ConnectNodes: ConnectNodes }
  | { DisconnectNodes: DisconnectNodes }
  | { ReplacePoints: ReplacePoints }
  | { InsertPoint: InsertPoint }
  | { RemovePoint: RemovePoint }
  | { MovePoint: MovePoint }
  | { ChangeCoefficient: ChangeCoefficient };
