/** 🔹 `replace-step-measure` mutation payload — whole-value swaps a process step's measure geometry. */
export interface ReplaceStepMeasure {
  id: string;
  newMeasure: unknown;
}

/** 🔖️ Semantic descriptor mirror: verb=`replace` entity=`step` kind=`replace-step-measure` record=`ReplacedStepMeasure`. */
export const ReplaceStepMeasureKind = "replace-step-measure" as const;
