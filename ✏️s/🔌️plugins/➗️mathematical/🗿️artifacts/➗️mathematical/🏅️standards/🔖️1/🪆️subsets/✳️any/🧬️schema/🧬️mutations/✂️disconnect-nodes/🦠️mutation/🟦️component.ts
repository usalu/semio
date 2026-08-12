/** ✂️ `disconnect-nodes` — removes an edge relationship between two graph nodes. */
export interface DisconnectNodes {
  id: string;
}

/** 🔖️ Semantic descriptor mirror: verb=`disconnect` entity=`node` kind=`disconnect-nodes` record=`DisconnectedNodes`. */
export const DisconnectNodesKind = "disconnect-nodes" as const;
