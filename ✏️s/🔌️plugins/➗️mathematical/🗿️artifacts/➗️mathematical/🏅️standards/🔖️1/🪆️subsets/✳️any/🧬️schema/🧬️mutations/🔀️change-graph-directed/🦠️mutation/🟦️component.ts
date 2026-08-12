/** 🔀️ `change-graph-directed` — flips the graph playground's directed/undirected toggle. */
export interface ChangeGraphDirected {
  newDirected: boolean;
}

/** 🔖️ Semantic descriptor mirror: verb=`change` entity=`graph` kind=`change-graph-directed` record=`ChangedGraphDirected`. */
export const ChangeGraphDirectedKind = "change-graph-directed" as const;
