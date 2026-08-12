/** 🔹 `create-step` mutation payload — adds a new process step. */
export interface CreateStep {
  index: number;
  step: unknown;
}

/** 🔖️ Semantic descriptor mirror: verb=`create` entity=`step` kind=`create-step` record=`CreatedStep`. */
export const CreateStepKind = "create-step" as const;
