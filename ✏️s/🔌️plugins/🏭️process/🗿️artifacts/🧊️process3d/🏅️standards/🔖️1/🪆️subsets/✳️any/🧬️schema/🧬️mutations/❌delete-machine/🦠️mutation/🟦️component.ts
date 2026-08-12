/** 🔹 `delete-machine` mutation payload — removes a workshop machine by id. */
export interface DeleteMachine {
  id: string;
}

/** 🔖️ Semantic descriptor mirror: verb=`delete` entity=`machine` kind=`delete-machine` record=`DeletedMachine`. */
export const DeleteMachineKind = "delete-machine" as const;
