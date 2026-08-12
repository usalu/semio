/** 🔁️ `replace-graph` — whole-value swap of the graph playground's structured payload (nodes, edges, algorithm, direction all at once) — the semantic replacement for the old generic `SetGraph`, used by gestures that load/paste an entire graph (e.g. the app's `SetArtifact` command) rather than editing one field or one node/edge. */
export interface ReplaceGraph {
  graph: unknown;
}

/** 🔖️ Semantic descriptor mirror: verb=`replace` entity=`graph` kind=`replace-graph` record=`ReplacedGraph`. */
export const ReplaceGraphKind = "replace-graph" as const;
