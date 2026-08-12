/** 🔹 `rename-step` mutation payload — sets a process step's display label. */
export interface RenameStep {
  id: string;
  newLabel: string;
}

/** 🔖️ Semantic descriptor mirror: verb=`rename` entity=`step` kind=`rename-step` record=`RenamedStep`. */
export const RenameStepKind = "rename-step" as const;
