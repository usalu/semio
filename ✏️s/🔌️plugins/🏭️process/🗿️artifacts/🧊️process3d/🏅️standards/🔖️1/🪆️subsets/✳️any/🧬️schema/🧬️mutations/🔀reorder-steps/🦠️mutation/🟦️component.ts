/** 🔹 `reorder-steps` mutation payload — repositions a process step within the timeline. */
export interface ReorderSteps {
  id: string;
  toIndex: number;
}

/** 🔖️ Semantic descriptor mirror: verb=`reorder` entity=`steps` kind=`reorder-steps` record=`ReorderedSteps`. */
export const ReorderStepsKind = "reorder-steps" as const;
