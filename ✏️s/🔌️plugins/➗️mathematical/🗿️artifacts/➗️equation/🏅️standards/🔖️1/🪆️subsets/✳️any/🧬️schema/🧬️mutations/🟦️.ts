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

/** 🚚️ `move-nodes` payload — one node-graph drag as intent: every addressed node moves by `(dx, dy)` from its base position. */
export interface MoveNodes {
  ids: string[];
  dx: number;
  dy: number;
}

/** 📌️ One graph node's absolute canvas position. */
export interface EquationNodePosition {
  id: string;
  x: number;
  y: number;
}

/** 📍️ `set-node-positions` payload — absolute positions of a set of graph nodes, the exact undo of a drag. */
export interface SetNodePositions {
  positions: EquationNodePosition[];
}

/** 🚪️ Parses one `move-nodes` payload the way its JSON Schema admits it, or throws. */
export function parseMoveNodes(value: unknown): MoveNodes {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new TypeError("move-nodes: payload is not an object");
  const row = value as Record<string, unknown>;
  const unknownKey = Object.keys(row).find((key) => !["ids", "dx", "dy"].includes(key));
  if (unknownKey !== undefined) throw new TypeError(`move-nodes: unknown field ${unknownKey}`);
  if (!Array.isArray(row.ids) || row.ids.length === 0 || row.ids.some((entry) => typeof entry !== "string" || entry.length === 0) || new Set(row.ids).size !== row.ids.length) throw new TypeError("move-nodes: ids must be a nonempty list of unique ids");
  if (typeof row.dx !== "number" || !Number.isFinite(row.dx) || typeof row.dy !== "number" || !Number.isFinite(row.dy)) throw new TypeError("move-nodes: dx and dy must be finite numbers");
  return { ids: row.ids as string[], dx: row.dx, dy: row.dy };
}

/** 🚪️ Parses one `set-node-positions` payload the way its JSON Schema (and its `unique-node-ids` invariant) admits it, or throws. */
export function parseSetNodePositions(value: unknown): SetNodePositions {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new TypeError("set-node-positions: payload is not an object");
  const row = value as Record<string, unknown>;
  if (Object.keys(row).some((key) => key !== "positions")) throw new TypeError("set-node-positions: unknown field");
  if (!Array.isArray(row.positions) || row.positions.length === 0) throw new TypeError("set-node-positions: positions must be a nonempty list");
  const positions = row.positions.map((entry): EquationNodePosition => {
    if (entry === null || typeof entry !== "object" || Array.isArray(entry)) throw new TypeError("set-node-positions: a position is not an object");
    const position = entry as Record<string, unknown>;
    if (Object.keys(position).some((key) => !["id", "x", "y"].includes(key))) throw new TypeError("set-node-positions: unknown position field");
    if (typeof position.id !== "string" || position.id.length === 0) throw new TypeError("set-node-positions: a position names a node id");
    if (typeof position.x !== "number" || !Number.isFinite(position.x) || typeof position.y !== "number" || !Number.isFinite(position.y)) throw new TypeError("set-node-positions: a position is two finite numbers");
    return { id: position.id, x: position.x, y: position.y };
  });
  if (new Set(positions.map((position) => position.id)).size !== positions.length) throw new TypeError("set-node-positions: each node at most once");
  return { positions };
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
  | { ChangeCoefficient: ChangeCoefficient }
  | { MoveNodes: MoveNodes }
  | { SetNodePositions: SetNodePositions };
