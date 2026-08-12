/** 🔹 `connect-nodes` mutation payload — creates an edge + relationship between two nodes. */
export interface ConnectNodes {
  edge: unknown;
  relationship: unknown;
}

/** 🔖️ Semantic descriptor mirror: verb=`connect` entity=`relationship` kind=`connect-nodes` record=`ConnectedNodes`. */
export const ConnectNodesKind = "connect-nodes" as const;
