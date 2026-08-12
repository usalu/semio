/** 🔹 `rename-machine` mutation payload — sets a workshop machine's display label. */
export interface RenameMachine {
  id: string;
  newLabel: string;
}

/** 🔖️ Semantic descriptor mirror: verb=`rename` entity=`machine` kind=`rename-machine` record=`RenamedMachine`. */
export const RenameMachineKind = "rename-machine" as const;
