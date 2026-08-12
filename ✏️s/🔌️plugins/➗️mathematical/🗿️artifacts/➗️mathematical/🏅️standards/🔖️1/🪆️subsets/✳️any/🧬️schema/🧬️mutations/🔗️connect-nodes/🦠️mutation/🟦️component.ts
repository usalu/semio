/** 🔗️ `connect-nodes` — creates an edge relationship between two graph nodes (the node-graph canvas's `connect` edit op). */
export interface ConnectNodes {
  id: string;
  source: string;
  target: string;
}

/** 🔖️ Semantic descriptor mirror: verb=`connect` entity=`node` kind=`connect-nodes` record=`ConnectedNodes`. */
export const ConnectNodesKind = "connect-nodes" as const;
