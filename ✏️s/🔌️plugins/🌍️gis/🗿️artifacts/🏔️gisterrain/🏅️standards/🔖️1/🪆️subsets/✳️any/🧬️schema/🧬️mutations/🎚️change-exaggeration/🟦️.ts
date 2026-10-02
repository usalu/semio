import type {Binary64} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";
/** 🎚️ `change-exaggeration` mutation payload — sets the terrain's vertical exaggeration scalar. */
export interface ChangeExaggeration {
  newExaggeration: Binary64;
}

/** 🔖️ Semantic descriptor mirror: verb=`change` entity=`exaggeration` kind=`change-exaggeration` record=`ChangedExaggeration`. */
export const ChangeExaggerationKind = "change-exaggeration" as const;
