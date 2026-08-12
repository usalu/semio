/** 🔹 `disconnect-nodes` mutation payload — removes an edge + relationship by id. */
export interface DisconnectNodes {
  edgeId: string;
}

/** 🔖️ Semantic descriptor mirror: verb=`disconnect` entity=`relationship` kind=`disconnect-nodes` record=`DisconnectedNodes`. */
export const DisconnectNodesKind = "disconnect-nodes" as const;
