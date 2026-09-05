/** 🎚️ `change-exaggeration` mutation payload — sets the terrain's vertical exaggeration scalar. */
export interface ChangeExaggeration {
  newExaggeration: number;
}

/** 🔖️ Semantic descriptor mirror: verb=`change` entity=`exaggeration` kind=`change-exaggeration` record=`ChangedExaggeration`. */
export const ChangeExaggerationKind = "change-exaggeration" as const;
