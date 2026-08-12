/** 🔹 `change-step-enabled` mutation payload — toggles whether a process step is enabled. */
export interface ChangeStepEnabled {
  id: string;
  newEnabled: boolean;
}

/** 🔖️ Semantic descriptor mirror: verb=`change` entity=`step` kind=`change-step-enabled` record=`ChangedStepEnabled`. */
export const ChangeStepEnabledKind = "change-step-enabled" as const;
