/** 🔹 `delete-step` mutation payload — removes a process step by id. */
export interface DeleteStep {
  id: string;
}

/** 🔖️ Semantic descriptor mirror: verb=`delete` entity=`step` kind=`delete-step` record=`DeletedStep`. */
export const DeleteStepKind = "delete-step" as const;
