/** ➗️ EquationMutation — closed semantic mutation vocabulary for the equation document,
 *  mirrors `🧬️mutations/🦀️.rs`'s `EquationMutation` enum and its 15 per-verb leaf
 *  structs. The enum carries NO `#[serde(tag = ...)]` — confirmed absent — so it serializes with
 *  serde's default EXTERNALLY TAGGED shape: `{ "<PascalCaseVariantName>": { ...leaf-struct-fields }
 *  }`, proven by every committed `🧪️tests/*​/🦠️mutation/🔣️.json` fixture (e.g.
 *  `{"ChangeNodeLabel":{"id":"n-alpha","new_label":"Alpha"}}`). NONE of the 15 leaf structs carry
 *  `#[serde(rename_all = ...)]` either, so every leaf's own field names are the literal Rust
 *  snake_case names verbatim (matches layout's convention, not present/note's camelCase leaves) —
 *  confirmed field-by-field against the committed fixtures. `EquationGraph`/`EquationPoint`
 *  themselves DO carry their own `rename_all = "camelCase"` (or have no snake_case fields), so
 *  `ReplaceGraph.graph.algorithmSeed` stays camelCase — that casing belongs to the referenced type,
 *  not to this leaf's own fields. */
import type { EquationGraph, EquationPoint } from "../🟦️.ts";

/** 🔢️ Addresses one numeric leaf in the equation tree — a `u64` newtype, wire-plain as a number. */
export type EquationNodeLabel = number;

/** 🔀️ `change-graph-directed` payload. */
export interface ChangeGraphDirected {
  new_directed: boolean;
}

/** 🧮️ `update-graph-algorithm` payload — algorithm id and seed are validated together. */
export interface UpdateGraphAlgorithm {
  new_algorithm: string;
  new_algorithm_seed: string | null;
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
  new_label: string;
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
