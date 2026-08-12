/** 🔹 `change-step-origin` mutation payload — sets a process step's machine/capability origin. */
export interface ChangeStepOrigin {
  id: string;
  newOrigin?: unknown;
}

/** 🔖️ Semantic descriptor mirror: verb=`change` entity=`step` kind=`change-step-origin` record=`ChangedStepOrigin`. */
export const ChangeStepOriginKind = "change-step-origin" as const;
