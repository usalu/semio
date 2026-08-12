/** 🏷️ `change-node-label` — sets a node's display label (`id` stays the stable identity field, so this is `change`, not `rename`). */
export interface ChangeNodeLabel {
  id: string;
  newLabel: string;
}

/** 🔖️ Semantic descriptor mirror: verb=`change` entity=`node` kind=`change-node-label` record=`ChangedNodeLabel`. */
export const ChangeNodeLabelKind = "change-node-label" as const;
