/** 🔹 `change-machine-icon` mutation payload — sets a workshop machine's icon. */
export interface ChangeMachineIcon {
  id: string;
  newIconId: string;
}

/** 🔖️ Semantic descriptor mirror: verb=`change` entity=`machine` kind=`change-machine-icon` record=`ChangedMachineIcon`. */
export const ChangeMachineIconKind = "change-machine-icon" as const;
